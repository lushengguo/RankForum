use crate::field::*;
use crate::generate_name;
use crate::post::*;
use crate::score::minimal_score_of_level;
use crate::user::*;
use crate::Address;

use lazy_static::lazy_static;
use log::{error, info, warn};
use rusqlite::{params, params_from_iter, Connection, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DB {
    conn: Mutex<rusqlite::Connection>,
}

lazy_static! {
    pub static ref GLOBAL_DB: Arc<DB> = {
        let db = DB::new("database.sqlite").expect("Failed to initialize database");
        db.init().expect("Failed to initialize database schema");
        Arc::new(db)
    };
}

pub fn global_db()-> Arc<DB> {
    GLOBAL_DB.clone()
}

impl DB {
    fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(DB { conn: Mutex::new(conn) })
    }

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
            name TEXT,
            score INTEGER NOT NULL
        )",
                params![],
            )?;
        }

        // Check and create 'post_and_comment' table
        let post_and_comment_table_exists: bool = self.conn.lock().unwrap().query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='post_and_comment');",
            params![],
            |row| row.get(0),
        )?;

        if !post_and_comment_table_exists {
            self.conn.lock().unwrap().execute(
                "CREATE TABLE IF NOT EXISTS post_and_comment (
            address TEXT PRIMARY KEY,
            from_address TEXT NOT NULL,
            to_address TEXT NOT NULL, 
            title TEXT NOT NULL, 
            content TEXT NOT NULL,
            score INTEGER NOT NULL,
            timestamp INTEGER NOT NULL,
            upvote INTEGER NOT NULL,
            downvote INTEGER NOT NULL
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
                        from_score_snapshot INTEGER NOT NULL,
                        to_score_snapshot INTEGER NOT NULL,
                        voted_score INTEGER NOT NULL
                    )",
                params![],
            )?;
        }

        Ok(())
    }

    pub fn upvote(
        &self,
        from: &Address,
        to: &Address,
        from_score: i64,
        to_score: i64,
        voted_score: i64,
    ) -> Result<(), String> {
        match self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO votes (to_address, from_address, from_score_snapshot, to_score_snapshot, voted_score) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![to, from, from_score, to_score, voted_score],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to save vote: {}", e);
                Err(e.to_string())
            }
        }
    }

    // voted score could be negative
    pub fn downvote(
        &self,
        from: &Address,
        to: &Address,
        from_score: i64,
        to_score: i64,
        voted_score: i64,
    ) -> Result<(), String> {
        self.upvote(from, to, from_score, to_score, voted_score)
    }

    pub fn rename(&self, address: Address, name: String) -> Result<(), String> {
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

    pub fn user(&self, name: Option<String>, address: Option<Address>) -> Option<User> {
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

    // pub fn user_in_field(&Option<>)

    pub fn all_fields(&self) -> Vec<Field> {
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

    pub fn score(&self, field_address: &Address, target_address: &Address) -> Option<i64> {
        match self.conn.lock().unwrap().query_row(
            "SELECT score FROM score WHERE field_address = ?1 AND address = ?2",
            params![field_address, target_address],
            |row| row.get(0),
        ) {
            Ok(score) => Some(score),
            Err(e) => {
                warn!("Failed to get score: {}", e);
                None
            }
        }
    }

    pub fn persist_score(&self, field: &String, address: &Address, name: Option<String>, score: i64) {
        match self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO score (address, field_address, name, score) VALUES (?1, ?2, ?3, ?4)",
            params![address, field, name, score],
        ) {
            Ok(_) => info!("Score saved"),
            Err(e) => error!("Failed to save score: {}", e),
        }
    }

    pub fn comment(&self, address: &Address) -> Option<Comment> {
        match self.conn.lock().unwrap().query_row(
            "SELECT address, from_address, to_address, content, score, timestamp, upvote, downvote 
            FROM post_and_comment WHERE address = ?1",
            params![address],
            |row| {
                Ok(Comment {
                    address: row.get(0)?,
                    from: row.get(1)?,
                    to: row.get(2)?,
                    content: row.get(3)?,
                    score: row.get(4)?,
                    timestamp: row.get(5)?,
                    upvote: row.get(6)?,
                    downvote: row.get(7)?,
                })
            },
        ) {
            Ok(comment) => Some(comment),
            Err(e) => {
                warn!("Failed to get comment by address: {}", e);
                None
            }
        }
    }

    fn create_user_if_not_exist(&self, address: &Address) -> Result<User, String> {
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

    // this allow anonymous user's post
    // and record this user in db with a random name
    pub fn persist_comment(&self, comment: &Comment) -> Result<(), String> {
        self.create_user_if_not_exist(&comment.from)?;
        self.post(comment.to.clone())?;

        match self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO post_and_comment (address, from_address, to_address, title, content, score, timestamp, upvote, downvote) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                comment.address,
                comment.from,
                comment.to,
                "".to_string(),
                comment.content,
                comment.score,
                comment.timestamp,
                comment.upvote,
                comment.downvote
            ],
        ) {
            Ok(_) => {
                info!("Comment saved");
                Ok(())
            }
            Err(e) => {
                error!("Failed to save comment: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub fn post(&self, address: Address) -> Result<Post, String> {
        match self.conn.lock().unwrap().query_row(
            "SELECT address, from_address, to_address, title, content, score, timestamp, upvote, downvote FROM post_and_comment WHERE address = ?1",
            params![address],
            |row| {
                Ok(Post {
                    address: row.get(0)?,
                    from: row.get(1)?,
                    to: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    score: row.get(5)?,
                    timestamp: row.get(6)?,
                    upvote: row.get(7)?,
                    downvote: row.get(8)?,
                    comments: HashMap::new(),
                })
            },
        ) {
            Ok(post) => Ok(post),
            Err(e) => {
                warn!("Failed to get post by address: {}", e);
                Err(e.to_string())
            }
        }
    }

    // this allow anonymous user's post
    // and record this user in db with a random name
    pub fn persist_post(&self, post: &Post) -> Result<(), String> {
        self.field(None, Some(post.to.clone()))?;
        self.create_user_if_not_exist(&post.from)?;

        match self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO post_and_comment (address, from_address, to_address, title, content, score, timestamp, upvote, downvote) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![post.address, post.from, post.to, post.title, post.content, post.score, post.timestamp, post.upvote, post.downvote],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to create new post: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub fn persist_field(&self, field: &Field) -> Result<(), String> {
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

    pub fn field(&self, name: Option<String>, address: Option<Address>) -> Result<Field, String> {
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
        let address = match self.field(Some(field.clone()), None) {
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

        let mut score = 0;
        if let Some(level) = option.level {
            sql.push_str(" AND score > ?");
            score = minimal_score_of_level(level);
            params.push(&score);
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
                    score: row.get(5)?,
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

    fn drop_db(db_path: &str) -> std::io::Result<()> {
        // remove database.sqlite
        std::fs::remove_file(db_path)
    }

    fn new_db() -> DB {
        let random_name = generate_name();
        let db_path = format!("target/{}.sqlite", random_name);

        let db = DB::new(&db_path).expect("Failed to initialize database");
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
        let persist_result = db.persist_field(&field);
        assert!(persist_result.is_ok());

        let field = db.field(Some(field.name.clone()), None).unwrap();
        assert_eq!(field.address, field.address);
    }

    #[test]
    fn test_register_and_rename_user() {
        let db = new_db();

        let user = User::new(generate_address(), generate_name());
        let register_result = db.rename(user.address.clone(), user.name.clone());
        assert!(register_result.is_ok());

        let user = db.user(Some(user.name.clone()), None).unwrap();
        assert_eq!(user.address, user.address);

        let new_name = generate_name();
        let rename_result = db.rename(user.address.clone(), new_name.clone());
        assert!(rename_result.is_ok());

        let user = db.user(None, Some(user.address.clone())).unwrap();
        assert_eq!(user.name, new_name);
    }

    fn create_field(db: &DB, address: &Address, name: &String) -> Result<Field, String> {
        let field = Field {
            address: address.clone(),
            name: name.clone(),
        };
        match db.persist_field(&field) {
            Ok(_) => {
                let field2 = db.field(Some(field.name.clone()), None).unwrap();
                assert!(field == field2);
                Ok(field)
            }
            Err(e) => Err(e),
        }
    }

    fn post(db: &DB, field_address: &Address) -> Result<Post, String> {
        let post = Post::new(
            generate_address(),
            field_address.clone(),
            generate_name(),
            generate_name(),
        );
        match db.persist_post(&post) {
            Ok(_) => {
                let post2 = db.post(post.address.clone()).unwrap();
                assert!(post == post2);
                Ok(post)
            }
            Err(e) => Err(e),
        }
    }

    fn comment(db: &DB, to: &Address) -> Result<Comment, String> {
        let comment = Comment {
            address: generate_address(),
            from: generate_address(),
            to: to.clone(),
            content: generate_name(),
            score: 0,
            timestamp: 0,
            upvote: 0,
            downvote: 0,
        };
        match db.persist_comment(&comment) {
            Ok(_) => {
                let comment2 = db.comment(&comment.address.clone()).unwrap();
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

        assert!(post(&db, &field.address).is_err());
    }

    #[test]
    fn test_post_on_exist_field() {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        assert!(post(&db, &field.address).is_ok());
    }

    #[test]
    fn test_comment_on_invalid_address() {
        let db = new_db();

        let result = comment(&db, &generate_address());
        assert!(result.is_err());
    }

    #[test]
    fn test_comment_on_post() {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        let post = post(&db, &field.address).unwrap();
        comment(&db, &post.address).unwrap();
    }

    #[test]
    fn test_comment_on_comment() {
        let db = new_db();

        let field = create_field(&db, &generate_address(), &generate_name()).unwrap();
        let post = post(&db, &field.address).unwrap();
        let comment1 = comment(&db, &post.address).unwrap();
        comment(&db, &comment1.address).unwrap();
    }

    #[test]
    fn test_upvote_downvote() {}
}
