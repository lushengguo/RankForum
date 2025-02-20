use crate::db_trait::Database;
use crate::field::Ordering;
use crate::field::*;
use crate::generate_unique_name;
use crate::post::*;
use crate::score::*;
use crate::textual_integer::TextualInteger;
use crate::user::*;
use crate::Address;

use lazy_static::lazy_static;
use log::{error, info, warn};
use rusqlite::{params, params_from_iter, Connection, Result};
use std::sync::{Arc, Mutex};

pub struct Sqlite {
    conn: Mutex<rusqlite::Connection>,
}

lazy_static! {
    static ref STATIC_DB: Arc<Sqlite> = {
        let db = Sqlite::new("database.sqlite").expect("Failed to initialize database");
        db.init().expect("Failed to initialize database schema");
        Arc::new(db)
    };
}

pub fn global_db() -> Arc<dyn Database> {
    STATIC_DB.clone()
}

impl Sqlite {
    fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Sqlite { conn: Mutex::new(conn) })
    }

    fn vote(
        &self,
        from: &Address,
        to: &Address,
        voted_score: TextualInteger,
        field_address: &str,
    ) -> Result<(), String> {
        let mut score = self.select_score(to, field_address);

        let mut db = self.conn.lock().unwrap();
        let tx = db.transaction().map_err(|e| e.to_string())?;

        match tx.query_row(
            "SELECT voted_score FROM votes WHERE from_address = ?1 AND to_address = ?2",
            params![from, to],
            |row| {
                let history_voted_score: TextualInteger = TextualInteger::new(&row.get::<_, String>(0)?);
                Ok(history_voted_score)
            },
        ) {
            Ok(history_voted_score) => {
                if history_voted_score.is_positive() == voted_score.is_positive() {
                    return Err("Already voted".to_string());
                } else {
                    tx.execute(
                        "UPDATE votes SET voted_score = ?1 WHERE from_address = ?2 AND to_address = ?3",
                        params![voted_score.to_string(), from, to],
                    )
                    .map_err(|err| err.to_string())?;

                    if voted_score.is_positive() {
                        score.upvote += 1;
                        score.downvote -= 1
                    } else {
                        score.upvote -= 1;
                        score.downvote += 1;
                    }

                    score.score += voted_score;
                    score.score -= history_voted_score;
                    self.update_score(&score, &tx)?;
                }
            }
            Err(_) => {
                tx.execute(
                    "INSERT OR REPLACE INTO votes (from_address, to_address, voted_score) 
            VALUES (?1, ?2, ?3)",
                    params![from, to, voted_score.to_string()],
                )
                .map_err(|err| err.to_string())?;

                if voted_score.is_positive() {
                    score.upvote += 1;
                } else {
                    score.downvote += 1;
                }
                score.score += voted_score;
                self.update_score(&score, &tx)?;
            }
        };

        tx.commit().map_err(|err| err.to_string())?;

        Ok(())
    }

    fn select_field_of_comment(&self, address: &Address) -> Result<Address, String> {
        let conn = self.conn.lock().unwrap();
        match conn.query_row(
            "SELECT address, field_address
            FROM score WHERE address = ?1",
            params![address],
            |row| Ok(row.get(1)?),
        ) {
            Ok(field_address) => Ok(field_address),
            Err(e) => {
                warn!("Failed to get field address by comment address: {}", e);
                Err(e.to_string())
            }
        }
    }

    fn select_or_insert_user(&self, address: &Address) -> Result<User, String> {
        let conn = self.conn.lock().unwrap();
        match conn.query_row("SELECT name FROM user WHERE address = ?1", params![address], |row| {
            row.get(0)
        }) {
            Ok(name) => Ok(User {
                address: address.clone(),
                name,
            }),
            Err(_) => {
                conn.execute(
                    "INSERT INTO user (address, name) VALUES (?1, ?2)",
                    params![address, generate_unique_name()],
                )
                .map_err(|err| err.to_string())?;

                Ok(User {
                    address: address.clone(),
                    name: generate_unique_name(),
                })
            }
        }
    }

    fn upsert_score(&self, score: &Score, tx: &rusqlite::Transaction) -> Result<(), String> {
        match tx.execute(
        "INSERT OR REPLACE INTO score (address, field_address, score, upvote, downvote) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            score.address,
            score.field_address,
            score.score.to_string(),
            score.upvote,
            score.downvote
        ],
    ) {
        Ok(_) => {
            info!("Score saved or updated");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save or update score: {}", e);
            Err(e.to_string())
        }
    }
    }

    fn update_score(&self, score: &Score, tx: &rusqlite::Transaction) -> Result<(), String> {
        match tx.execute(
            "UPDATE score SET score = ?1, upvote = ?2, downvote = ?3 WHERE address = ?4 AND field_address = ?5",
            params![
                score.score.to_string(),
                score.upvote,
                score.downvote,
                score.address,
                score.field_address
            ],
        ) {
            Ok(_) => {
                info!("Score updated");
                Ok(())
            }
            Err(e) => {
                error!("Failed to update score: {}", e);
                Err(e.to_string())
            }
        }
    }
    fn sort_comments_candidate(&self, comments: &mut Vec<Comment>, option: &FilterOption) {
        if option.ordering == Ordering::ByTimestamp {
            return;
        }

        match option.ordering {
            Ordering::ByScore => {
                comments.sort_by(|a, b| a.score.cmp(&b.score));
            }
            Ordering::ByUpVote => {
                comments.sort_by(|a, b| a.upvote.cmp(&b.upvote));
            }
            Ordering::ByDownVote => {
                comments.sort_by(|a, b| a.downvote.cmp(&b.downvote));
            }
            Ordering::ByUpvoteSubDownVote => {
                comments.sort_by(|a, b| {
                    (a.upvote as i128 - a.downvote as i128).cmp(&(b.upvote as i128 - b.downvote as i128))
                });
            }
            _ => {}
        }
        if !option.ascending {
            comments.reverse();
        }
    }

    fn filter_comment_by_level(&self, comments: &mut Vec<Comment>, _level: u8) {
        comments.retain(|comment| {
            let score = self.select_score(&comment.address, &comment.field_address);
            level(&score.score) >= _level
        });
    }

    fn fill_comment_score(&self, comment: &mut Comment) {
        let score = self.select_score(&comment.address, &comment.field_address);
        comment.score = score.score;
        comment.upvote = score.upvote;
        comment.downvote = score.downvote;
    }

    fn sort_posts_candidate(&self, posts: &mut Vec<Post>, option: &FilterOption) {
        if option.ordering == Ordering::ByTimestamp {
            return;
        }

        match option.ordering {
            Ordering::ByScore => {
                posts.sort_by(|a, b| a.score.cmp(&b.score));
            }
            Ordering::ByUpVote => {
                posts.sort_by(|a, b| a.upvote.cmp(&b.upvote));
            }
            Ordering::ByDownVote => {
                posts.sort_by(|a, b| a.downvote.cmp(&b.downvote));
            }
            Ordering::ByUpvoteSubDownVote => {
                posts.sort_by(|a, b| {
                    (a.upvote as i128 - a.downvote as i128).cmp(&(b.upvote as i128 - b.downvote as i128))
                });
            }
            _ => {}
        }
        if !option.ascending {
            posts.reverse();
        }
    }

    fn filter_post_by_level(&self, posts: &mut Vec<Post>, _level: u8) {
        posts.retain(|post| {
            let score = self.select_score(&post.address, &post.to);
            level(&score.score) >= _level
        });
    }

    fn fill_post_score(&self, post: &mut Post) {
        let score = self.select_score(&post.address, &post.to);
        post.score = score.score;
        post.upvote = score.upvote;
        post.downvote = score.downvote;
    }
}

impl Database for Sqlite {
    /// Initializes the database schema by creating necessary tables if they do not exist.
    ///
    /// # Tables
    ///
    /// ## `user`
    /// | Column  | Type | Constraints     |
    /// |---------|------|-----------------|
    /// | address | TEXT | PRIMARY KEY     |
    /// | name    | TEXT | NOT NULL        |
    ///
    /// ## `fields`
    /// | Column  | Type | Constraints     |
    /// |---------|------|-----------------|
    /// | address | TEXT | PRIMARY KEY     |
    /// | name    | TEXT | NOT NULL        |
    ///
    /// ## `score`
    /// | Column        | Type    | Constraints     |
    /// |---------------|---------|-----------------|
    /// | address       | TEXT    | PRIMARY KEY     |
    /// | field_address | TEXT    | NOT NULL        |
    /// | score         | TEXT | NOT NULL        |
    /// | upvote        | INTEGER | NOT NULL        |
    /// | downvote      | INTEGER | NOT NULL        |
    ///
    /// ## `post`
    /// | Column       | Type    | Constraints     |
    /// |--------------|---------|-----------------|
    /// | address      | TEXT    | PRIMARY KEY     |
    /// | from_address | TEXT    | NOT NULL        |
    /// | to_address   | TEXT    | NOT NULL        |
    /// | title        | TEXT    | NOT NULL        |
    /// | content      | TEXT    | NOT NULL        |
    /// | timestamp    | INTEGER | NOT NULL        |
    ///
    /// ## `comment`
    /// | Column       | Type    | Constraints     |
    /// |--------------|---------|-----------------|
    /// | address      | TEXT    | PRIMARY KEY     |
    /// | from_address | TEXT    | NOT NULL        |
    /// | to_address   | TEXT    | NOT NULL        |
    /// | field_address| TEXT    | NOT NULL        |
    /// | content      | TEXT    | NOT NULL        |
    /// | timestamp    | INTEGER | NOT NULL        |
    ///
    /// ## `votes`
    /// | Column              | Type    | Constraints     |
    /// |---------------------|---------|-----------------|
    /// | to_address          | TEXT    | NOT NULL        |
    /// | from_address        | TEXT    | NOT NULL        |
    /// | voted_score         | TEXT    | NOT NULL        |
    ///
    fn init(&self) -> Result<(), String> {
        // Check and create 'user' table
        let user_table_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='user');",
                params![],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;

        if !user_table_exists {
            self.conn
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS user (
                    address TEXT PRIMARY KEY, 
                    name TEXT NOT NULL
                )",
                    params![],
                )
                .map_err(|err| err.to_string())?;
        }

        // Check and create 'fields' table
        let fields_table_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='fields');",
                params![],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;

        if !fields_table_exists {
            self.conn
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS fields (
                    address TEXT PRIMARY KEY, 
                    name TEXT NOT NULL
                )",
                    params![],
                )
                .map_err(|err| err.to_string())?;
        }

        // Check and create 'score' table
        let score_table_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='score');",
                params![],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;

        if !score_table_exists {
            self.conn
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS score (
            address TEXT PRIMARY KEY,
            field_address TEXT NOT NULL,
            score TEXT NOT NULL,
            upvote INTEGER NOT NULL,
            downvote INTEGER NOT NULL
        )",
                    params![],
                )
                .map_err(|err| err.to_string())?;
        }

        // Check and create 'post' table
        let post_table_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='post');",
                params![],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;

        if !post_table_exists {
            self.conn
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS post (
            address TEXT PRIMARY KEY,
            from_address TEXT NOT NULL,
            to_address TEXT NOT NULL, 
            title TEXT NOT NULL, 
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
                    params![],
                )
                .map_err(|err| err.to_string())?;
        }

        // Check and create 'comment' table
        let comment_table_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='comment');",
                params![],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;

        if !comment_table_exists {
            self.conn
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS comment (
                    address TEXT PRIMARY KEY,
                    from_address TEXT NOT NULL,
                    to_address TEXT NOT NULL, 
                    field_address TEXT NOT NULL, 
                    content TEXT NOT NULL,
                    timestamp INTEGER NOT NULL
                )",
                    params![],
                )
                .map_err(|err| err.to_string())?;
        }

        // Check and create 'votes' table
        let votes_table_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='votes');",
                params![],
                |row| row.get(0),
            )
            .map_err(|err| err.to_string())?;

        if !votes_table_exists {
            self.conn
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS votes (
                        from_address TEXT NOT NULL,
                        to_address TEXT NOT NULL,
                        voted_score TEXT NOT NULL
                    )",
                    params![],
                )
                .map_err(|err| err.to_string())?;
        }

        Ok(())
    }

    fn upvote(
        &self,
        from: &Address,
        to: &Address,
        voted_score: TextualInteger,
        field_address: &str,
    ) -> Result<(), String> {
        self.vote(from, to, voted_score, field_address)
    }

    // voted score could be negative
    fn downvote(
        &self,
        from: &Address,
        to: &Address,
        voted_score: TextualInteger,
        field_address: &str,
    ) -> Result<(), String> {
        self.vote(from, to, voted_score, field_address)
    }

    fn upsert_user(&self, address: Address, name: String) -> Result<(), String> {
        let name_exists: bool = self
            .conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT EXISTS(SELECT * FROM user WHERE name=(?1))",
                params![name],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())
            .unwrap();

        if name_exists {
            return Err("Name already exists".to_string());
        }

        match self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO user (address, name) VALUES (?1, ?2)",
            params![address, name],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to create new user: {}", e);
                Err(e.to_string())
            }
        }
    }

    fn select_user(&self, name: Option<String>, address: Option<Address>) -> Option<User> {
        match self.conn.lock().unwrap().query_row(
            "SELECT name, address FROM user WHERE name = ?1 OR address = ?2",
            params![name, address],
            |row| {
                Ok(User {
                    name: row.get(0)?,
                    address: row.get(1)?,
                })
            },
        ) {
            Ok(user) => Some(user),
            Err(e) => {
                warn!("Failed to get user by name or address: {}", e);
                None
            }
        }
    }

    fn select_score(&self, address: &str, field_address: &str) -> Score {
        let conn = self.conn.lock().unwrap();
        match conn.query_row(
            "SELECT address, field_address, score, upvote, downvote FROM score WHERE address = ?1 AND field_address = ?2",
            params![address, field_address],
            |row| {
                Ok(Score {
                    address: row.get(0)?,
                    field_address: row.get(1)?,
                    score: TextualInteger::new(&row.get::<_, String>(2)?),
                    upvote: row.get(3)?,
                    downvote: row.get(4)?,
                })
            },
        ) {
            Ok(score) => score,
            Err(e) => {
                Score { address:address.to_string(), field_address: field_address.to_string(), score: TextualInteger::new("0"), upvote: 0, downvote: 0 }
            }
        }
    }

    fn select_all_fields(&self) -> Vec<Field> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT address, name FROM fields").unwrap();
        let field_iter = stmt.query_map([], |row| {
            Ok(Field {
                address: row.get(0)?,
                name: row.get(1)?,
            })
        });

        let mut fields = Vec::new();
        for field in field_iter.unwrap() {
            fields.push(field.unwrap());
        }

        fields
    }

    fn select_comment(&self, address: &Address) -> Result<Comment, String> {
        let field_address = self.select_field_of_comment(&address)?;
        let score = self.select_score(address, &field_address);

        let db = self.conn.lock().unwrap();
        match db.query_row(
            "SELECT address, from_address, to_address, content, timestamp, field_address
            FROM comment WHERE address = ?1",
            params![address],
            |row| {
                Ok(Comment {
                    address: row.get(0)?,
                    from: row.get(1)?,
                    to: row.get(2)?,
                    content: row.get(3)?,
                    score: score.score,
                    timestamp: row.get(4)?,
                    upvote: score.upvote,
                    downvote: score.downvote,
                    field_address: row.get(5)?,
                    comments: Vec::new(),
                })
            },
        ) {
            Ok(comment) => Ok(comment),
            Err(e) => {
                warn!("Failed to get comment by address: {}", e);
                Err(e.to_string())
            }
        }
    }

    fn upsert_comment(&self, comment: &Comment) -> Result<(), String> {
        self.select_or_insert_user(&comment.from)?;
        let post_result = self.select_post(&comment.to.clone());
        let comment_result = self.select_comment(&comment.to.clone());
        if post_result.is_err() && comment_result.is_err() {
            return Err("invalid to address".to_string());
        }

        if post_result.is_ok() {
            let post = post_result.unwrap();
            if post.to != comment.field_address {
                return Err("Post field address not match".to_string());
            }
        }

        if comment_result.is_ok() {
            let comment = comment_result.unwrap();
            if comment.field_address != comment.field_address {
                return Err("Comment field address not match".to_string());
            }
        }

        let mut db = self.conn.lock().unwrap();

        // automatically rollback on drop
        let tx = db.transaction().map_err(|e| e.to_string())?;

        let score = Score {
            address: comment.address.clone(),
            field_address: comment.field_address.clone(),
            score: comment.score.clone(),
            upvote: comment.upvote,
            downvote: comment.downvote,
        };
        self.upsert_score(&score, &tx)?;

        match tx.execute(
            "INSERT OR REPLACE INTO comment (address, from_address, to_address, field_address, content, timestamp) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                comment.address,
                comment.from,
                comment.to,
                comment.field_address,
                comment.content,
                comment.timestamp,
            ],
        ) {
            Ok(_) => {
                info!("Comment saved");
                tx.commit().map_err(|e| e.to_string())?;
                Ok(())
            }
            Err(e) => {
                error!("Failed to save comment: {}", e);
                tx.rollback().map_err(|e| e.to_string())?;
                Err(e.to_string())
            }
        }
    }

    fn select_post(&self, address: &str) -> Result<Post, String> {
        let mut post = match self.conn.lock().unwrap().query_row(
            "SELECT address, from_address, to_address, title, content, timestamp FROM post WHERE address = ?1",
            params![address],
            |row| {
                Ok(Post {
                    address: row.get(0)?,
                    from: row.get(1)?,
                    to: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    score: TextualInteger::new("0"),
                    timestamp: row.get(5)?,
                    upvote: 0,
                    downvote: 0,
                    comments: Vec::new(),
                })
            },
        ) {
            Ok(post) => post,
            Err(e) => return Err(e.to_string()),
        };

        let score = self.select_score(&post.address, &post.to);
        post.score = score.score;
        post.upvote = score.upvote;
        post.downvote = score.downvote;
        Ok(post)
    }

    // this allow anonymous user's post
    // and record this user in db with a random name
    fn upsert_post(&self, post: &Post) -> Result<(), String> {
        self.select_field(None, Some(post.to.clone()))?;
        self.select_or_insert_user(&post.from)?;

        let mut db = self.conn.lock().unwrap();

        // automatically rollback on drop
        let tx = db.transaction().map_err(|e| e.to_string())?;

        let score = Score {
            address: post.address.clone(),
            field_address: post.to.clone(),
            score: post.score.clone(),
            upvote: post.upvote,
            downvote: post.downvote,
        };
        self.upsert_score(&score, &tx)?;

        match tx.execute(
            "INSERT OR REPLACE INTO post (address, from_address, to_address, title, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![post.address, post.from, post.to, post.title, post.content, post.timestamp],
        ) {
            Ok(_) => {tx.commit().map_err(|err|err.to_string())?;
                Ok(())},
            Err(e) => {
                error!("Failed to create new post: {}", e);
                tx.rollback().map_err(|err|err.to_string())?;
                Err(e.to_string())
            }
        }
    }

    fn insert_field(&self, field: &Field) -> Result<(), String> {
        match self.conn.lock().unwrap().execute(
            "INSERT INTO fields (address, name) VALUES (?1, ?2)",
            params![field.address, field.name],
        ) {
            Ok(_) => {
                info!("Field saved");
                Ok(())
            }
            Err(e) => {
                error!("Failed to save field: {}", e);
                Err(e.to_string())
            }
        }
    }

    fn select_field(&self, name: Option<String>, address: Option<Address>) -> Result<Field, String> {
        if name.is_some() {
            match self.conn.lock().unwrap().query_row(
                "SELECT address, name FROM fields WHERE name = ?1",
                params![name],
                |row| {
                    Ok(Field {
                        address: row.get(0)?,
                        name: row.get(1)?,
                    })
                },
            ) {
                Ok(field) => {
                    if address.is_some() && field.address != address.unwrap() {
                        warn!("Field address not match");
                        Err("Field address not match".to_string())
                    } else {
                        Ok(field)
                    }
                }
                Err(e) => {
                    warn!("Failed to get field by name: {}", e);
                    Err(e.to_string())
                }
            }
        } else {
            match self.conn.lock().unwrap().query_row(
                "SELECT address, name FROM fields WHERE address = ?1",
                params![address],
                |row| {
                    Ok(Field {
                        address: row.get(0)?,
                        name: row.get(1)?,
                    })
                },
            ) {
                Ok(field) => Ok(field),
                Err(e) => {
                    warn!("Failed to get field by address: {}", e);
                    Err(e.to_string())
                }
            }
        }
    }

    fn field_by_address(&self, comment_or_post_id: &Address) -> Option<Field> {
        match self.conn.lock().unwrap().query_row(
            "SELECT address, name FROM fields WHERE address = ?1",
            params![comment_or_post_id],
            |row| {
                Ok(Field {
                    address: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        ) {
            Ok(field) => Some(field),
            Err(e) => {
                warn!("Failed to get field by address: {}", e);
                None
            }
        }
    }

    fn filter_comments(&self, to: &Address, option: &FilterOption) -> Result<Vec<Comment>, String> {
        let mut sql = "SELECT address, from_address, to_address, field_address, content, timestamp FROM comment WHERE to_address = ?"
            .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&to];

        let mut keyword = String::new();
        if option.keyword.is_some() {
            keyword = format!("%{}%", option.keyword.clone().unwrap());
            sql.push_str(" AND content LIKE ?");
            params.push(&keyword);
        }

        if option.ordering == Ordering::ByTimestamp {
            sql.push_str(" ORDER BY timestamp");
            if !option.ascending {
                sql.push_str(" DESC");
            }
        }

        let mut comments = Vec::new();
        {
            let conn = self.conn.lock().unwrap();
            let mut stmt = conn.prepare(&sql).map_err(|err| err.to_string())?;
            let comment_iter = stmt
                .query_map(params_from_iter(params.iter()), |row| {
                    Ok(Comment {
                        address: row.get(0)?,
                        from: row.get(1)?,
                        to: row.get(2)?,
                        field_address: row.get(3)?,
                        content: row.get(4)?,
                        timestamp: row.get(5)?,
                        score: TextualInteger::new("0"),
                        upvote: 0,
                        downvote: 0,
                        comments: Vec::new(),
                    })
                })
                .unwrap();

            for comment in comment_iter {
                comments.push(comment.unwrap());
            }
        }

        for comment in comments.iter_mut() {
            self.fill_comment_score(comment);
        }

        self.sort_comments_candidate(&mut comments, option);
        if option.level.is_some() {
            self.filter_comment_by_level(&mut comments, option.level.unwrap());
        }

        comments.truncate(option.max_results as usize);

        Ok(comments)
    }

    fn filter_posts(&self, to: &Address, option: &FilterOption) -> Result<Vec<Post>, String> {
        let mut sql =
            "SELECT address, from_address, to_address, title, content, timestamp FROM post WHERE to_address = ?"
                .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&to];

        let mut keyword = String::new();
        if option.keyword.is_some() {
            keyword = format!("%{}%", option.keyword.clone().unwrap());
            sql.push_str(" AND (content LIKE ? OR title LIKE ?)");
            params.push(&keyword);
            params.push(&keyword);
        }

        if option.ordering == Ordering::ByTimestamp {
            sql.push_str(" ORDER BY timestamp");
            if !option.ascending {
                sql.push_str(" DESC");
            }
        }

        let mut posts = Vec::new();
        {
            let conn = self.conn.lock().unwrap();
            let mut stmt = conn.prepare(&sql).map_err(|err| err.to_string())?;
            let post_iter = stmt
                .query_map(params_from_iter(params.iter()), |row| {
                    Ok(Post {
                        address: row.get(0)?,
                        from: row.get(1)?,
                        to: row.get(2)?,
                        title: row.get(3)?,
                        content: row.get(4)?,
                        timestamp: row.get(5)?,
                        score: TextualInteger::new("0"),
                        upvote: 0,
                        downvote: 0,
                        comments: Vec::new(),
                    })
                })
                .unwrap();

            for post in post_iter {
                posts.push(post.unwrap());
            }
        }

        for post in posts.iter_mut() {
            self.fill_post_score(post);
        }

        self.sort_posts_candidate(&mut posts, option);
        if option.level.is_some() {
            self.filter_post_by_level(&mut posts, option.level.unwrap());
        }

        posts.truncate(option.max_results as usize);

        Ok(posts)
    }
}
