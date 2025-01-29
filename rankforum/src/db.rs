use crate::field::*;
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
    conn: rusqlite::Connection,
}

lazy_static! {
    pub static ref GLOBAL_DB: Arc<Mutex<DB>> = Arc::new(Mutex::new(
        DB::new("database.sqlite").expect("Failed to initialize database")
    ));
}

impl DB {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(DB { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                address TEXT PRIMARY KEY, 
                name TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS fields (
                address TEXT PRIMARY KEY, 
                name TEXT NOT NULL,
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS score (
                address TEXT PRIMARY KEY,
                field_address TEXT NOT NULL,
                name TEXT,
                score INTEGER NOT NULL
            )",
            [],
        )?;

        // post and comment
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS post (
                address TEXT PRIMARY KEY,
                from TEXT NOT NULL,
                to TEXT NOT NULL, 
                title TEXT, 
                content TEXT NOT NULL,
                score INTEGER NOT NULL,
                timestamp INTEGER NOT NULL,
                upvote INTEGER NOT NULL,
                downvote INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn new_user(address: Address, name: String) -> Result<(), String> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.execute(
            "INSERT INTO user (address, name) VALUES (?1, ?2)",
            params![address, name],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to create new user: {}", e);
                Err(e.to_string())
            }
        }
    }

    pub fn user(name: Option<String>, address: Option<Address>) -> Option<User> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.query_row(
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

    pub fn all_fields() -> Vec<Field> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        let mut stmt = _db.conn.prepare("SELECT address, name FROM fields").unwrap();
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

    pub fn score(field_address: &Address, target_address: &Address) -> Option<i64> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.query_row(
            "SELECT score FROM score WHERE field_address = ?1 AND address = ?2",
            params![field_address, target_address],
            |row| Ok(row.get(0)?),
        ) {
            Ok(score) => Some(score),
            Err(e) => {
                warn!("Failed to get score: {}", e);
                None
            }
        }
    }

    pub fn update_score(field: &String, address: &Address, name: Option<String>, score: i64) {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.execute(
            "INSERT OR REPLACE score (address, field_address, name, score) VALUES (?1, ?2, ?3, ?4)",
            params![address, field, name, score],
        ) {
            Ok(_) => info!("Score saved"),
            Err(e) => error!("Failed to save score: {}", e),
        }
    }

    pub fn comment(address: &Address) -> Option<Comment> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.query_row(
            "SELECT address, from, to, content, score, timestamp, upvote, downvote FROM comment WHERE address = ?1",
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

    pub fn update_comment(comment: &Comment) {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.execute(
            "INSERT OR REPLACE comment (address, from, to, content, score, timestamp, upvote, downvote) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![comment.address, comment.from, comment.to, comment.content, comment.score, comment.timestamp, comment.upvote, comment.downvote],
        ) {
            Ok(_) => info!("Comment saved"),
            Err(e) => error!("Failed to save comment: {}", e),
        }
    }

    pub fn post(address: Address) -> Option<Post> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.query_row(
            "SELECT address, from, to, title, content, score, timestamp, upvote, downvote FROM post WHERE address = ?1",
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
            Ok(post) => Some(post),
            Err(e) => {
                warn!("Failed to get post by address: {}", e);
                None
            }
        }
    }

    pub fn save_field(field: &Field) {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.execute(
            "INSERT OR REPLACE fields (address, name) VALUES (?1, ?2)",
            params![field.address, field.name],
        ) {
            Ok(_) => info!("Field saved"),
            Err(e) => error!("Failed to save field: {}", e),
        }
    }

    pub fn field(name: Option<String>, address: Option<Address>) -> Option<Field> {
        if name.is_some() && address.is_some() {
            return None;
        }

        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.query_row(
            "SELECT address, name FROM fields WHERE name = ?1 OR address = ?2",
            params![name, address],
            |row| {
                Ok(Field {
                    address: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        ) {
            Ok(field) => Some(field),
            Err(e) => {
                warn!("Failed to get field by name or address: {}", e);
                None
            }
        }
    }

    pub fn field_by_address(comment_or_post_id: &Address) -> Option<Field> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.query_row(
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

    // this would not update comments
    pub fn update_post(post: &Post) {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        match _db.conn.execute(
            "INSERT OR REPLACE post (address, from, to, content, score, timestamp, upvote, downvote) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![post.address, post.from, post.to, post.content, post.score, post.timestamp, post.upvote, post.downvote],
        ) {
            Ok(_) => info!("Post saved"),
            Err(e) => error!("Failed to save post: {}", e),
        }
    }

    pub fn filter_posts(field: &String, option: &FilterOption) -> Vec<Post> {
        let db = GLOBAL_DB.clone();
        let _db = db.lock().unwrap();
        let address = match DB::field(Some(field.clone()), None) {
            Some(field) => field.address,
            None => {
                warn!("Field not found");
                return vec![];
            }
        };

        let mut sql =
            "SELECT address, from, to, title, content, score, timestamp, upvote, downvote FROM post WHERE to = ?"
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
        if let Some(_) = &option.keyword {
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

        let mut stmt = _db.conn.prepare(&sql).unwrap();
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
