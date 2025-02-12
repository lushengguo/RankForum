use crate::field::*;
use crate::generate_name;
use crate::post::*;
use crate::score::*;
use crate::textual_integer::TextualInteger;
use crate::user::*;
use crate::Address;

use lazy_static::lazy_static;
use log::{error, info, warn};
use rusqlite::{params, params_from_iter, Connection, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DB {
    conn: Mutex<rusqlite::Connection>,

    db_path: String,
    rm_db_on_drop: bool,
}

lazy_static! {
    pub static ref GLOBAL_DB: Arc<DB> = {
        let db = DB::new("database.sqlite", false).expect("Failed to initialize database");
        db.init().expect("Failed to initialize database schema");
        Arc::new(db)
    };
}

pub fn global_db() -> Arc<DB> {
    GLOBAL_DB.clone()
}

impl Drop for DB {
    fn drop(&mut self) {
        if self.rm_db_on_drop {
            if let Err(e) = std::fs::remove_file(&self.db_path) {
                error!("Failed to remove database file: {}", e);
            }
        }
    }
}

impl DB {
    fn new(path: &str, rm_db_on_drop: bool) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(DB {
            conn: Mutex::new(conn),
            db_path: path.to_string(),
            rm_db_on_drop,
        })
    }

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
    /// | to_address          | TEXT    | PRIMARY KEY     |
    /// | from_address        | TEXT    | NOT NULL        |
    /// | from_score_snapshot | TEXT    | NOT NULL        |
    /// | to_score_snapshot   | TEXT    | NOT NULL        |
    /// | voted_score         | TEXT    | NOT NULL        |
    ///
    pub fn init(&self) -> Result<()> {
        // Check and create 'user' table
        let user_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='user');",
            params![],
            |row| row.get(0),
        )?;

        if !user_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS user (
                    address TEXT PRIMARY KEY, 
                    name TEXT NOT NULL
                )",
                params![],
            )?;
        }

        // Check and create 'fields' table
        let fields_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='fields');",
            params![],
            |row| row.get(0),
        )?;

        if !fields_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS fields (
                    address TEXT PRIMARY KEY, 
                    name TEXT NOT NULL
                )",
                params![],
            )?;
        }

        // Check and create 'score' table
        let score_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='score');",
            params![],
            |row| row.get(0),
        )?;

        if !score_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS score (
            address TEXT PRIMARY KEY,
            field_address TEXT NOT NULL,
            score TEXT NOT NULL,
            upvote INTEGER NOT NULL,
            downvote INTEGER NOT NULL
        )",
                params![],
            )?;
        }

        // Check and create 'post' table
        let post_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='post');",
            params![],
            |row| row.get(0),
        )?;

        if !post_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS post (
            address TEXT PRIMARY KEY,
            from_address TEXT NOT NULL,
            to_address TEXT NOT NULL, 
            title TEXT NOT NULL, 
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
                params![],
            )?;
        }

        // Check and create 'comment' table
        let comment_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='comment');",
            params![],
            |row| row.get(0),
        )?;

        if !comment_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS comment (
                    address TEXT PRIMARY KEY,
                    from_address TEXT NOT NULL,
                    to_address TEXT NOT NULL, 
                    field_address TEXT NOT NULL, 
                    content TEXT NOT NULL,
                    timestamp INTEGER NOT NULL
                )",
                params![],
            )?;
        }

        // Check and create 'votes' table
        let votes_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='votes');",
            params![],
            |row| row.get(0),
        )?;

        if !votes_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS votes (
                        to_address TEXT PRIMARY KEY,
                        from_address TEXT NOT NULL,
                        from_score_snapshot TEXT NOT NULL,
                        to_score_snapshot TEXT NOT NULL,
                        voted_score TEXT NOT NULL
                    )",
                params![],
            )?;
        }

        Ok(())
    }

    fn vote(
        &self,
        from: &Address,
        to: &Address,
        from_score: TextualInteger,
        to_score: TextualInteger,
        voted_score: TextualInteger,
        field_address: &String,
    ) -> Result<(), String> {
        let mut score = self.select_score(to, field_address)?;

        let mut db = self.conn.lock().unwrap();
        let tx = db.transaction().map_err(|e| e.to_string())?;

        tx.execute(
            "INSERT OR REPLACE INTO votes (to_address, from_address, from_score_snapshot, to_score_snapshot, voted_score) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![to, from, from_score.to_string(), to_score.to_string(), voted_score.to_string()],
        ).map_err(|err|err.to_string())?;

        if voted_score.is_positive() {
            score.upvote += 1;
        } else {
            score.downvote += 1;
        }
        score.score += voted_score;
        self.update_score(&score, &tx)?;

        tx.commit().map_err(|err| err.to_string())?;

        Ok(())
    }

    pub fn upvote(
        &self,
        from: &Address,
        to: &Address,
        from_score: TextualInteger,
        to_score: TextualInteger,
        voted_score: TextualInteger,
        field_address: &String,
    ) -> Result<(), String> {
        self.vote(from, to, from_score, to_score, voted_score, field_address)
    }

    // voted score could be negative
    pub fn downvote(
        &self,
        from: &Address,
        to: &Address,
        from_score: TextualInteger,
        to_score: TextualInteger,
        voted_score: TextualInteger,
        field_address: &String,
    ) -> Result<(), String> {
        self.vote(from, to, from_score, to_score, voted_score, field_address)
    }

    pub fn rename_user(&self, address: Address, name: String) -> Result<(), String> {
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

    pub fn select_user(&self, name: Option<String>, address: Option<Address>) -> Option<User> {
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

    pub fn select_score(&self, address: &String, field_address: &String) -> Result<Score, String> {
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
            Ok(score) => Ok(score),
            Err(e) => {
                error!("Failed to get score: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub fn select_all_fields(&self) -> Vec<Field> {
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

    pub fn select_comment(&self, address: &Address) -> Result<Comment, String> {
        let field_address = self.select_field_of_comment(&address)?;
        let score = self.select_score(address, &field_address)?;

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
                    params![address, generate_name()],
                )
                .map_err(|err| err.to_string())?;

                Ok(User {
                    address: address.clone(),
                    name: generate_name(),
                })
            }
        }
    }

    fn insert_score(&self, score: &Score, tx: &rusqlite::Transaction) -> Result<(), String> {
        match tx.execute(
            "INSERT INTO score (address, field_address, score, upvote, downvote) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                score.address,
                score.field_address,
                score.score.to_string(),
                score.upvote,
                score.downvote
            ],
        ) {
            Ok(_) => {
                info!("Score saved");
                Ok(())
            }
            Err(e) => {
                error!("Failed to save score: {}", e);
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

    pub fn insert_comment(&self, comment: &Comment) -> Result<(), String> {
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
            score: TextualInteger::new("0"),
            upvote: 0,
            downvote: 0,
        };
        self.insert_score(&score, &tx)?;

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

    pub fn select_post(&self, address: &String) -> Result<Post, String> {
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
                    comments: HashMap::new(),
                })
            },
        ) {
            Ok(post) => post,
            Err(e) => return Err(e.to_string()),
        };

        let score = self.select_score(&post.address, &post.to)?;
        post.score = score.score;
        post.upvote = score.upvote;
        post.downvote = score.downvote;
        Ok(post)
    }

    // this allow anonymous user's post
    // and record this user in db with a random name
    pub fn insert_post(&self, post: &Post) -> Result<(), String> {
        self.select_field(None, Some(post.to.clone()))?;
        self.select_or_insert_user(&post.from)?;

        let mut db = self.conn.lock().unwrap();

        // automatically rollback on drop
        let tx = db.transaction().map_err(|e| e.to_string())?;

        let score = Score {
            address: post.address.clone(),
            field_address: post.to.clone(),
            score: TextualInteger::new("0"),
            upvote: 0,
            downvote: 0,
        };
        self.insert_score(&score, &tx)?;

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

    pub fn insert_field(&self, field: &Field) -> Result<(), String> {
        match self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO fields (address, name) VALUES (?1, ?2)",
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

    pub fn select_field(&self, name: Option<String>, address: Option<Address>) -> Result<Field, String> {
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

    pub fn field_by_address(&self, comment_or_post_id: &Address) -> Option<Field> {
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

    pub fn filter_posts(&self, field: &String, option: &FilterOption) -> Vec<Post> {
        let address = match self.select_field(Some(field.clone()), None) {
            Ok(field) => field.address,
            Err(e) => {
                warn!("Field not found, error: {}", e);
                return vec![];
            }
        };

        let mut sql =
            "SELECT address, from_address, to_address, title, content, score, timestamp, upvote, downvote FROM post WHERE to = ?"
                .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = vec![&address];
        params.push(&address);

        let mut score = TextualInteger::new("0");
        let mut score_str = String::new();
        if let Some(level) = option.level {
            sql.push_str(" AND score > ?");
            score = minimal_score_of_level(level);
            score_str = score.to_string();
            params.push(&score_str);
        }

        let keyword_param = format!("%{}%", option.keyword.clone().unwrap());
        if option.keyword.is_some() {
            sql.push_str(" AND (title LIKE ? OR content LIKE ?)");
            params.push(&keyword_param);
            params.push(&keyword_param);
        }

        if option.ascending_by_timestamp {
            sql.push_str(" ORDER BY timestamp ASC");
        } else {
            sql.push_str(" ORDER BY timestamp DESC");
        }

        if option.ascending_by_absolute_score {
            sql.push_str(", ABS(score) ASC");
        } else {
            sql.push_str(", ABS(score) DESC");
        }

        sql.push_str(" LIMIT ?");
        params.push(&option.max_results);

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql).unwrap();
        let post_iter = stmt
            .query_map(params_from_iter(params.iter()), |row| {
                Ok(Post {
                    address: row.get(0)?,
                    from: row.get(1)?,
                    to: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    score: TextualInteger::new(&row.get::<_, String>(5)?),
                    timestamp: row.get(6)?,
                    upvote: row.get(7)?,
                    downvote: row.get(8)?,
                    comments: HashMap::new(),
                })
            })
            .unwrap();

        let mut posts = Vec::new();
        for post in post_iter {
            posts.push(post.unwrap());
        }

        posts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_address;
    use crate::generate_name;

    fn new_db() -> DB {
        let random_name = generate_name();
        let db_path = format!("target/{}.sqlite", random_name);

        let db = DB::new(&db_path, true).expect("Failed to initialize database");
        db.init().expect("Failed to initialize database schema");

        db
    }

    #[test]
    fn test_create_field() {
        let db = new_db();

        let field = Field {
            address: generate_address(),
            name: generate_name(),
        };
        let insert_result = db.insert_field(&field);
        assert!(insert_result.is_ok());

        let field = db.select_field(Some(field.name.clone()), None).unwrap();
        assert_eq!(field.address, field.address);
    }

    #[test]
    fn test_register_and_rename_user() {
        let db = new_db();

        let user = User::new(generate_address(), generate_name());
        let register_result = db.rename_user(user.address.clone(), user.name.clone());
        assert!(register_result.is_ok());

        let user = db.select_user(Some(user.name.clone()), None).unwrap();
        assert_eq!(user.address, user.address);

        let new_name = generate_name();
        let rename_result = db.rename_user(user.address.clone(), new_name.clone());
        assert!(rename_result.is_ok());

        let user = db.select_user(None, Some(user.address.clone())).unwrap();
        assert_eq!(user.name, new_name);
    }

    fn create_field(db: &DB, address: &Address, name: &String) -> Result<Field, String> {
        let field = Field {
            address: address.clone(),
            name: name.clone(),
        };
        match db.insert_field(&field) {
            Ok(_) => {
                let field2 = db.select_field(Some(field.name.clone()), None).unwrap();
                assert!(field == field2);
                Ok(field)
            }
            Err(e) => Err(e),
        }
    }

    fn insert_post(db: &DB, field_address: &Address) -> Result<Post, String> {
        let post = Post::new(
            generate_address(),
            field_address.clone(),
            generate_name(),
            generate_name(),
        );
        match db.insert_post(&post) {
            Ok(_) => {
                let post2 = db.select_post(&post.address).unwrap();
                assert!(post == post2);
                Ok(post)
            }
            Err(e) => Err(e),
        }
    }

    fn insert_comment(db: &DB, to: &Address, field_address: &Address) -> Result<Comment, String> {
        let comment = Comment {
            address: generate_address(),
            from: generate_address(),
            to: to.clone(),
            content: generate_name(),
            score: TextualInteger::new("0"),
            timestamp: 0,
            upvote: 0,
            downvote: 0,
            field_address: field_address.clone(),
        };
        match db.insert_comment(&comment) {
            Ok(_) => {
                let comment2 = db.select_comment(&comment.address.clone()).unwrap();
                assert!(comment == comment2);
                Ok(comment)
            }
            Err(e) => Err(e),
        }
    }

    #[test]
    fn test_post_on_not_exist_field() {
        let db = new_db();

        let field = Field {
            address: generate_address(),
            name: generate_name(),
        };

        assert!(insert_post(&db, &field.address).is_err());
    }

    #[test]
    fn test_post_on_exist_field() {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        assert!(insert_post(&db, &field.address).is_ok());
    }

    #[test]
    fn test_comment_on_invalid_address() {
        let db = new_db();

        let result: std::result::Result<Comment, String> =
            insert_comment(&db, &generate_address(), &generate_address());
        assert!(result.is_err());
    }

    #[test]
    fn test_comment_on_post() {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        let post = insert_post(&db, &field.address).unwrap();
        insert_comment(&db, &post.address, &post.to).unwrap();
    }

    #[test]
    fn test_comment_on_comment() {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        let post = insert_post(&db, &field.address).unwrap();
        let comment1 = insert_comment(&db, &post.address, &post.to).unwrap();
        insert_comment(&db, &comment1.address, &post.to).unwrap();
    }

    fn assert_user_score_eqs(db: &DB, field: &Field, user_address: &Address, score: TextualInteger) {
        match db.select_score(user_address, &field.address) {
            Ok(user_score) => assert_eq!(user_score.score, score),
            Err(_) => assert_eq!(TextualInteger::new("0"), score),
        }
    }

    fn assert_post_score_eqs(db: &DB, field: &Field, post_address: &Address, score: TextualInteger) {
        let post_score = db.select_score(&post_address, &field.address).unwrap().score;
        assert_eq!(post_score, score);
    }

    fn assert_comment_sore_equals(db: &DB, field: &Field, comment_address: &Address, score: TextualInteger) {
        let comment_score = db.select_score(&comment_address, &field.address).unwrap().score;
        assert_eq!(comment_score, score);
    }

    fn init_field_user_post_comment() -> (DB, Field, Post, Comment, User) {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        let post = insert_post(&db, &field.address).unwrap();
        let comment = insert_comment(&db, &post.address, &post.to).unwrap();
        let user = User::new(generate_address(), generate_name());

        assert_user_score_eqs(&db, &field, &user.address, TextualInteger::new("0"));
        assert_post_score_eqs(&db, &field, &post.address, TextualInteger::new("0"));
        assert_comment_sore_equals(&db, &field, &comment.address, TextualInteger::new("0"));

        return (db, field, post, comment, user);
    }

    #[test]
    fn test_upvote_on_post() {
        let (db, field, post, _, user) = init_field_user_post_comment();
        db.upvote(
            &user.address,
            &post.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&post.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("1"));
    }

    #[test]
    fn test_downvote_on_post() {
        let (db, field, post, _, user) = init_field_user_post_comment();
        db.downvote(
            &user.address,
            &post.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("-1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&post.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("-1"));
    }

    #[test]
    fn test_upvote_on_comment() {
        let (db, field, _, comment, user) = init_field_user_post_comment();
        db.upvote(
            &user.address,
            &comment.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&comment.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("1"));
    }

    #[test]
    fn test_downvote_on_comment() {
        let (db, field, _, comment, user) = init_field_user_post_comment();
        db.downvote(
            &user.address,
            &comment.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("-1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&comment.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("-1"));
    }

    #[test]
    fn test_score_down_cross_zero() {
        let (db, field, _, comment, user) = init_field_user_post_comment();

        db.upvote(
            &user.address,
            &comment.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&comment.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("1"));

        db.downvote(
            &user.address,
            &comment.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("-1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&comment.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("0"));

        db.downvote(
            &user.address,
            &comment.address,
            TextualInteger::new("0"),
            TextualInteger::new("0"),
            TextualInteger::new("-1"),
            &field.address,
        )
        .unwrap();
        let score = db.select_score(&comment.address, &field.address).unwrap();
        assert_eq!(score.score, TextualInteger::new("-1"));
    }
}
