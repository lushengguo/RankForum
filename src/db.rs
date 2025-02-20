use crate::db_sqlite;
use crate::db_trait::Database;
use crate::field::Ordering;
use crate::field::*;
use crate::generate_unique_name;
use crate::post::*;
use crate::score::*;
use crate::textual_integer::TextualInteger;
use crate::user::*;
use crate::Address;
use std::sync::Arc;

enum DbType {
    Sqlite,
}

impl DbType {
    const fn values() -> &'static [DbType] {
        &[DbType::Sqlite]
    }
}

pub fn default_global_db() -> Arc<dyn Database> {
    global_db(&DbType::Sqlite)
}

pub fn global_db(db_type: &DbType) -> Arc<dyn Database> {
    match db_type {
        DbType::Sqlite => {
            return db_sqlite::global_db();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_unique_address;
    use crate::generate_unique_name;

    #[test]
    fn test_create_field() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = Field {
                address: generate_unique_address(),
                name: generate_unique_name(),
            };
            let insert_result = db.insert_field(&field);
            assert!(insert_result.is_ok());

            let field = db.select_field(Some(field.name.clone()), None).unwrap();
            assert_eq!(field.address, field.address);
        }
    }

    #[test]
    fn test_register_and_rename_user() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let user = User::new(generate_unique_address(), generate_unique_name());
            let register_result = db.upsert_user(user.address.clone(), user.name.clone());
            assert!(register_result.is_ok());

            let user = db.select_user(Some(user.name.clone()), None).unwrap();
            assert_eq!(user.address, user.address);

            let new_name = generate_unique_name();
            let rename_result = db.upsert_user(user.address.clone(), new_name.clone());
            assert!(rename_result.is_ok());

            let user = db.select_user(None, Some(user.address.clone())).unwrap();
            assert_eq!(user.name, new_name);
        }
    }

    fn create_field(db: Arc<dyn Database>, address: &Address, name: &str) -> Result<Field, String> {
        let field = Field {
            address: address.clone(),
            name: name.to_string(),
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

    fn upsert_post(db: Arc<dyn Database>, field_address: &Address) -> Result<Post, String> {
        let post = Post::new(
            generate_unique_address(),
            field_address.clone(),
            generate_unique_name(),
            generate_unique_name(),
        );
        match db.upsert_post(&post) {
            Ok(_) => {
                let post2 = db.select_post(&post.address).unwrap();
                assert!(post == post2);
                Ok(post)
            }
            Err(e) => Err(e),
        }
    }

    fn upsert_comment(db: Arc<dyn Database>, to: &Address, field_address: &Address) -> Result<Comment, String> {
        let comment = Comment {
            address: generate_unique_address(),
            from: generate_unique_address(),
            to: to.clone(),
            content: generate_unique_name(),
            score: TextualInteger::new("0"),
            timestamp: 0,
            upvote: 0,
            downvote: 0,
            field_address: field_address.clone(),
            comments: Vec::new(),
        };
        match db.upsert_comment(&comment) {
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
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = Field {
                address: generate_unique_address(),
                name: generate_unique_name(),
            };

            assert!(upsert_post(db.clone(), &field.address).is_err());
        }
    }

    #[test]
    fn test_post_on_exist_field() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            assert!(upsert_post(db.clone(), &field.address).is_ok());
        }
    }

    #[test]
    fn test_comment_on_invalid_address() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let result: std::result::Result<Comment, String> =
                upsert_comment(db.clone(), &generate_unique_address(), &generate_unique_address());
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_comment_on_post() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post = upsert_post(db.clone(), &field.address).unwrap();
            upsert_comment(db.clone(), &post.address, &post.to).unwrap();
        }
    }

    #[test]
    fn test_comment_on_comment() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post = upsert_post(db.clone(), &field.address).unwrap();
            let comment1 = upsert_comment(db.clone(), &post.address, &post.to).unwrap();
            upsert_comment(db.clone(), &comment1.address, &post.to).unwrap();
        }
    }

    fn assert_user_score_eqs(db: Arc<dyn Database>, field: &Field, user_address: &Address, score: TextualInteger) {
        assert_eq!(db.select_score(user_address, &field.address).score, score);
    }

    fn assert_post_score_eqs(db: Arc<dyn Database>, field: &Field, post_address: &Address, score: TextualInteger) {
        let post_score = db.select_score(&post_address, &field.address).score;
        assert_eq!(post_score, score);
    }

    fn assert_comment_sore_equals(
        db: Arc<dyn Database>,
        field: &Field,
        comment_address: &Address,
        score: TextualInteger,
    ) {
        let comment_score = db.select_score(&comment_address, &field.address).score;
        assert_eq!(comment_score, score);
    }

    fn init_field_user_post_comment(db_type: &DbType) -> (Arc<dyn Database>, Field, Post, Comment, User) {
        let db = global_db(db_type);

        let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
        let post = upsert_post(db.clone(), &field.address).unwrap();
        let comment = upsert_comment(db.clone(), &post.address, &post.to).unwrap();
        let user = User::new(generate_unique_address(), generate_unique_name());

        assert_user_score_eqs(db.clone(), &field, &user.address, TextualInteger::new("0"));
        assert_post_score_eqs(db.clone(), &field, &post.address, TextualInteger::new("0"));
        assert_comment_sore_equals(db.clone(), &field, &comment.address, TextualInteger::new("0"));

        return (db, field, post, comment, user);
    }

    #[test]
    fn test_upvote_on_post() {
        for db_type in DbType::values() {
            let (db, field, post, _, user) = init_field_user_post_comment(db_type);
            db.upvote(&user.address, &post.address, TextualInteger::new("1"), &field.address)
                .unwrap();
            let score = db.select_score(&post.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));
        }
    }

    #[test]
    fn test_downvote_on_post() {
        for db_type in DbType::values() {
            let (db, field, post, _, user) = init_field_user_post_comment(db_type);
            db.downvote(&user.address, &post.address, TextualInteger::new("-1"), &field.address)
                .unwrap();
            let score = db.select_score(&post.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("-1"));
        }
    }

    #[test]
    fn test_upvote_on_comment() {
        for db_type in DbType::values() {
            let (db, field, _, comment, user) = init_field_user_post_comment(db_type);
            db.upvote(
                &user.address,
                &comment.address,
                TextualInteger::new("1"),
                &field.address,
            )
            .unwrap();
            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));
        }
    }

    #[test]
    fn test_downvote_on_comment() {
        for db_type in DbType::values() {
            let (db, field, _, comment, user) = init_field_user_post_comment(db_type);
            db.downvote(
                &user.address,
                &comment.address,
                TextualInteger::new("-1"),
                &field.address,
            )
            .unwrap();
            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("-1"));
        }
    }

    #[test]
    fn test_score_down_cross_zero() {
        for db_type in DbType::values() {
            let (db, field, _, comment, user) = init_field_user_post_comment(db_type);

            db.upvote(
                &user.address,
                &comment.address,
                TextualInteger::new("1"),
                &field.address,
            )
            .unwrap();
            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));

            db.downvote(
                &user.address,
                &comment.address,
                TextualInteger::new("-1"),
                &field.address,
            )
            .unwrap();
            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("-1"));

            let result = db.downvote(
                &user.address,
                &comment.address,
                TextualInteger::new("-1"),
                &field.address,
            );
            assert!(result.is_err());

            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("-1"));
        }
    }

    #[test]
    fn test_double_vote() {
        for db_type in DbType::values() {
            let (db, field, post, comment, user) = init_field_user_post_comment(db_type);
            // post
            db.upvote(&user.address, &post.address, TextualInteger::new("1"), &field.address)
                .unwrap();
            let score = db.select_score(&post.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));

            let result = db.upvote(&user.address, &post.address, TextualInteger::new("1"), &field.address);
            assert!(result.is_err());
            let score = db.select_score(&post.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));

            db.upvote(&user.address, &post.address, TextualInteger::new("-1"), &field.address)
                .unwrap();

            let score = db.select_score(&post.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("-1"));

            // comment
            db.upvote(
                &user.address,
                &comment.address,
                TextualInteger::new("1"),
                &field.address,
            )
            .unwrap();
            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));

            let result = db.upvote(
                &user.address,
                &comment.address,
                TextualInteger::new("1"),
                &field.address,
            );
            assert!(result.is_err());
            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("1"));

            db.upvote(
                &user.address,
                &comment.address,
                TextualInteger::new("-1"),
                &field.address,
            )
            .unwrap();

            let score = db.select_score(&comment.address, &field.address);
            assert_eq!(score.score, TextualInteger::new("-1"));
        }
    }

    fn make_comment(
        db: Arc<dyn Database>,
        post: &Post,
        score: TextualInteger,
        timestamp: i64,
        upvote: u64,
        downvote: u64,
        content: &str,
    ) -> Comment {
        let comment = Comment {
            address: generate_unique_address(),
            from: generate_unique_address(),
            to: post.address.clone(),
            content: content.to_string(),
            score: score.clone(),
            timestamp: timestamp,
            upvote: upvote,
            downvote: downvote,
            field_address: post.to.clone(),
            comments: Vec::new(),
        };
        db.upsert_comment(&comment).unwrap();
        comment
    }

    #[test]
    fn test_filter_comments_ordering() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post = upsert_post(db.clone(), &field.address).unwrap();

            let comment1 = make_comment(db.clone(), &post, TextualInteger::new("1"), 2, 3, 4, "");
            let comment2 = make_comment(db.clone(), &post, TextualInteger::new("2"), 3, 4, 1, "");
            let comment3 = make_comment(db.clone(), &post, TextualInteger::new("3"), 4, 1, 2, "");
            let comment4 = make_comment(db.clone(), &post, TextualInteger::new("4"), 1, 2, 3, "");

            let mut filter_option = FilterOption {
                level: None,
                keyword: None,
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 10,
            };
            assert_eq!(
                db.filter_comments(&post.address, &filter_option).unwrap(),
                vec![comment4.clone(), comment1.clone(), comment2.clone(), comment3.clone()]
            );

            filter_option.ordering = Ordering::ByScore;
            assert_eq!(
                db.filter_comments(&post.address, &filter_option).unwrap(),
                vec![comment1.clone(), comment2.clone(), comment3.clone(), comment4.clone()]
            );

            filter_option.ordering = Ordering::ByUpVote;
            assert_eq!(
                db.filter_comments(&post.address, &filter_option).unwrap(),
                vec![comment3.clone(), comment4.clone(), comment1.clone(), comment2.clone()]
            );

            filter_option.ordering = Ordering::ByDownVote;
            assert_eq!(
                db.filter_comments(&post.address, &filter_option).unwrap(),
                vec![comment2.clone(), comment3.clone(), comment4.clone(), comment1.clone()]
            );

            // -1 3 -1 -1
            filter_option.ordering = Ordering::ByUpvoteSubDownVote;
            filter_option.ascending = false;
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments[0], comment2);

            filter_option.ordering = Ordering::ByTimestamp;
            filter_option.ascending = false;
            assert_eq!(
                db.filter_comments(&post.address, &filter_option).unwrap(),
                vec![comment3.clone(), comment2.clone(), comment1.clone(), comment4.clone()]
            );
        }
    }

    #[test]
    fn test_filter_comment_level() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post = upsert_post(db.clone(), &field.address).unwrap();

            let comment1 = make_comment(db.clone(), &post, TextualInteger::new("1"), 0, 0, 0, "");
            let comment2 = make_comment(db.clone(), &post, TextualInteger::new("100"), 1, 0, 0, "");
            let comment3 = make_comment(db.clone(), &post, TextualInteger::new("10000"), 2, 0, 0, "");
            let comment4 = make_comment(db.clone(), &post, TextualInteger::new("1000000"), 3, 0, 0, "");

            let mut filter_option = FilterOption {
                level: Some(0),
                keyword: None,
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 10,
            };

            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 4);
            assert_eq!(
                comments,
                vec![comment1.clone(), comment2.clone(), comment3.clone(), comment4.clone()]
            );

            filter_option.level = Some(1);
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 3);
            assert_eq!(comments, vec![comment2.clone(), comment3.clone(), comment4.clone()]);

            filter_option.level = Some(2);
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 2);

            filter_option.level = Some(3);
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 1);
        }
    }

    #[test]
    fn test_filter_comment_keyword() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post = upsert_post(db.clone(), &field.address).unwrap();
            let comment1 = make_comment(db.clone(), &post, TextualInteger::new("1"), 0, 0, 0, "test comment 1");
            let comment2 = make_comment(
                db.clone(),
                &post,
                TextualInteger::new("1"),
                1,
                0,
                0,
                "another test comment 2",
            );
            let comment3 = make_comment(db.clone(), &post, TextualInteger::new("1"), 2, 0, 0, "comment 3");
            let comment4 = make_comment(
                db.clone(),
                &post,
                TextualInteger::new("1"),
                3,
                0,
                0,
                "test keyword comment 4",
            );

            let mut filter_option = FilterOption {
                level: None,
                keyword: Some("test".to_string()),
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 10,
            };

            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 3);
            assert_eq!(comments, vec![comment1.clone(), comment2.clone(), comment4.clone()]);

            filter_option.keyword = Some("another".to_string());
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 1);
            assert_eq!(comments, vec![comment2.clone()]);

            filter_option.keyword = Some("comment 3".to_string());
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 1);
            assert_eq!(comments, vec![comment3.clone()]);

            filter_option.keyword = Some("nonexistent".to_string());
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 0);
        }
    }

    #[test]
    fn test_filter_comment_limit() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post = upsert_post(db.clone(), &field.address).unwrap();
            let comment1 = make_comment(db.clone(), &post, TextualInteger::new("1"), 0, 0, 0, "");
            let comment2 = make_comment(db.clone(), &post, TextualInteger::new("1"), 1, 0, 0, "");
            let comment3 = make_comment(db.clone(), &post, TextualInteger::new("1"), 2, 0, 0, "");
            let comment4 = make_comment(db.clone(), &post, TextualInteger::new("1"), 3, 0, 0, "");

            let mut filter_option = FilterOption {
                level: None,
                keyword: None,
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 0,
            };

            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 0);

            filter_option.max_results = 1;
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 1);
            assert_eq!(comments, vec![comment1.clone()]);

            filter_option.max_results = 2;
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 2);
            assert_eq!(comments, vec![comment1.clone(), comment2.clone()]);

            filter_option.max_results = 3;
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 3);
            assert_eq!(comments, vec![comment1.clone(), comment2.clone(), comment3.clone()]);

            filter_option.max_results = 4;
            let comments = db.filter_comments(&post.address, &filter_option).unwrap();
            assert_eq!(comments.len(), 4);
            assert_eq!(
                comments,
                vec![comment1.clone(), comment2.clone(), comment3.clone(), comment4.clone()]
            );
        }
    }

    fn make_post(
        db: Arc<dyn Database>,
        field: &Field,
        score: TextualInteger,
        timestamp: i64,
        upvote: u64,
        downvote: u64,
        title: &str,
        content: &str,
    ) -> Post {
        let post = Post {
            address: generate_unique_address(),
            from: generate_unique_address(),
            to: field.address.clone(),
            title: title.to_string(),
            content: content.to_string(),
            score: score.clone(),
            timestamp: timestamp,
            upvote: upvote,
            downvote: downvote,
            comments: Vec::new(),
        };
        db.upsert_post(&post).unwrap();
        post
    }

    #[test]
    fn test_filter_post_ordering() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post1 = make_post(db.clone(), &field, TextualInteger::new("1"), 2, 3, 4, "", "");
            let post2 = make_post(db.clone(), &field, TextualInteger::new("2"), 3, 4, 1, "", "");
            let post3 = make_post(db.clone(), &field, TextualInteger::new("3"), 4, 1, 2, "", "");
            let post4 = make_post(db.clone(), &field, TextualInteger::new("4"), 1, 2, 3, "", "");

            let mut filter_option = FilterOption {
                level: None,
                keyword: None,
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 10,
            };
            assert_eq!(
                db.filter_posts(&field.address, &filter_option).unwrap(),
                vec![post4.clone(), post1.clone(), post2.clone(), post3.clone()]
            );

            filter_option.ordering = Ordering::ByScore;
            assert_eq!(
                db.filter_posts(&field.address, &filter_option).unwrap(),
                vec![post1.clone(), post2.clone(), post3.clone(), post4.clone()]
            );

            filter_option.ordering = Ordering::ByUpVote;
            assert_eq!(
                db.filter_posts(&field.address, &filter_option).unwrap(),
                vec![post3.clone(), post4.clone(), post1.clone(), post2.clone()]
            );

            filter_option.ordering = Ordering::ByDownVote;
            assert_eq!(
                db.filter_posts(&field.address, &filter_option).unwrap(),
                vec![post2.clone(), post3.clone(), post4.clone(), post1.clone()]
            );

            // -1 3 -1 -1
            filter_option.ordering = Ordering::ByUpvoteSubDownVote;
            filter_option.ascending = false;
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts[0], post2);

            filter_option.ordering = Ordering::ByTimestamp;
            filter_option.ascending = false;
            assert_eq!(
                db.filter_posts(&field.address, &filter_option).unwrap(),
                vec![post3.clone(), post2.clone(), post1.clone(), post4.clone()]
            );
        }
    }

    #[test]
    fn test_filter_post_level() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post1 = make_post(db.clone(), &field, TextualInteger::new("1"), 0, 0, 0, "", "");
            let post2 = make_post(db.clone(), &field, TextualInteger::new("100"), 1, 0, 0, "", "");
            let post3 = make_post(db.clone(), &field, TextualInteger::new("10000"), 2, 0, 0, "", "");
            let post4 = make_post(db.clone(), &field, TextualInteger::new("1000000"), 3, 0, 0, "", "");

            let mut filter_option = FilterOption {
                level: Some(0),
                keyword: None,
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 10,
            };

            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 4);
            assert_eq!(posts, vec![post1.clone(), post2.clone(), post3.clone(), post4.clone()]);

            filter_option.level = Some(1);
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 3);
            assert_eq!(posts, vec![post2.clone(), post3.clone(), post4.clone()]);

            filter_option.level = Some(2);
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 2);

            filter_option.level = Some(3);
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 1);
        }
    }

    #[test]
    fn test_filter_post_keyword() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post1 = make_post(db.clone(), &field, TextualInteger::new("1"), 0, 0, 0, "test post 1", "");
            let post2 = make_post(
                db.clone(),
                &field,
                TextualInteger::new("1"),
                1,
                0,
                0,
                "another test post 2",
                "",
            );
            let post3 = make_post(db.clone(), &field, TextualInteger::new("1"), 2, 0, 0, "post 3", "");
            let post4 = make_post(
                db.clone(),
                &field,
                TextualInteger::new("1"),
                3,
                0,
                0,
                "test keyword post 4",
                "",
            );

            let mut filter_option = FilterOption {
                level: None,
                keyword: Some("test".to_string()),
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 10,
            };

            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 3);
            assert_eq!(posts, vec![post1.clone(), post2.clone(), post4.clone()]);

            filter_option.keyword = Some("another".to_string());
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 1);
            assert_eq!(posts, vec![post2.clone()]);

            filter_option.keyword = Some("post 3".to_string());
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 1);
            assert_eq!(posts, vec![post3.clone()]);

            filter_option.keyword = Some("nonexistent".to_string());
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 0);
        }
    }

    #[test]
    fn test_filter_post_limit() {
        for db_type in DbType::values() {
            let db = global_db(db_type);

            let field = create_field(db.clone(), &generate_unique_address(), &generate_unique_name()).unwrap();
            let post1 = make_post(db.clone(), &field, TextualInteger::new("1"), 0, 0, 0, "", "");
            let post2 = make_post(db.clone(), &field, TextualInteger::new("1"), 1, 0, 0, "", "");
            let post3 = make_post(db.clone(), &field, TextualInteger::new("1"), 2, 0, 0, "", "");
            let post4 = make_post(db.clone(), &field, TextualInteger::new("1"), 3, 0, 0, "", "");

            let mut filter_option = FilterOption {
                level: None,
                keyword: None,
                ordering: Ordering::ByTimestamp,
                ascending: true,
                max_results: 0,
            };

            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 0);

            filter_option.max_results = 1;
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 1);
            assert_eq!(posts, vec![post1.clone()]);

            filter_option.max_results = 2;
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 2);
            assert_eq!(posts, vec![post1.clone(), post2.clone()]);

            filter_option.max_results = 3;
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 3);
            assert_eq!(posts, vec![post1.clone(), post2.clone(), post3.clone()]);

            filter_option.max_results = 4;
            let posts = db.filter_posts(&field.address, &filter_option).unwrap();
            assert_eq!(posts.len(), 4);
            assert_eq!(posts, vec![post1.clone(), post2.clone(), post3.clone(), post4.clone()]);
        }
    }
}
