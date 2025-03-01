use crate::crypto::*;
use crate::db::default_global_db;
use crate::post::*;
use crate::user::*;
use crate::Address;
use crate::field::{Field, FilterOption, Ordering};
use base64::prelude::*;
use lazy_static::lazy_static;
use rouille::*;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::db_trait::Database;
use crate::generate_unique_address;
use serde_json;

lazy_static! {
    static ref GLOBAL_SESSION_STORGE: Mutex<HashMap<String, SessionStorage>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct SessionStorage {
    logined: bool,
    address: Address,
}

pub fn handle_route(request: &Request) -> Response {
    if request.url() != "login" && request.method() == "POST" && !user_already_logined(request) {
        return rouille::Response::text("please login first").with_status_code(401);
    }

    router!(request,
        (POST) (/login) => {
            login(request)
        },
        (POST) (/post) => {
            post(request)
        },
        (POST) (/comment) => {
            comment(request)
        },
        (GET) (/filter_post) => {
            filter_post(request)
        },
        (POST) (/rename_user) => {
            user_rename(request)
        },
        (POST) (/create_user) => {
            create_user(request)
        },
        (POST) (/upvote) => {
            upvote(request)
        },
        (POST) (/downvote) => {
            downvote(request)
        },
        (GET) (/query_user_address) => {
            query_user_address(request)
        },
        (GET) (/query_field_address) => {
            query_field_address(request)
        },
        (GET) (/query_score_in_field) => {
            query_score_in_field(request)
        },
        _ => rouille::Response::empty_404()
    )
}

fn get_session_cache(request: &Request) -> Option<SessionStorage> {
    let sid = match request.get_param("SID") {
        Some(sid) => sid,
        None => return None,
    };

    let sessions_storage = GLOBAL_SESSION_STORGE.lock().unwrap();
    sessions_storage.get(&sid).cloned()
}

fn address(request: &Request) -> Option<Address> {
    match get_session_cache(request) {
        Some(cache) => Some(cache.address),
        None => None,
    }
}

fn user_already_logined(request: &Request) -> bool {
    match get_session_cache(request) {
        Some(cache) => cache.logined,
        None => false,
    }
}

fn query_user_address(request: &Request) -> Response {
    let user_name = request.get_param("user_name").unwrap_or("".to_string());
    if user_name.is_empty() {
        return Response::text("missing required parameter user_name").with_status_code(400);
    }

    let user = default_global_db().select_user(Some(user_name), None);
    if user.is_none() {
        return Response::text("user not found").with_status_code(404);
    }

    Response::text(user.unwrap().address)
}

fn query_field_address(request: &Request) -> Response {
    let field_name = request.get_param("field_name").unwrap_or("".to_string());
    if field_name.is_empty() {
        return Response::text("missing required parameter field_name").with_status_code(400);
    }

    let field = default_global_db().select_field(Some(field_name), None);
    if field.is_err() {
        return Response::text("field not found").with_status_code(404);
    }

    Response::text(field.unwrap().address)
}

fn query_score_in_field(request: &Request) -> Response {
    let user_name = request.get_param("user_name").unwrap_or("".to_string());
    let user_address = address(request).unwrap();
    let field_name = request.get_param("field_name").unwrap_or("".to_string());
    let field_address = request.get_param("field_address").unwrap_or("".to_string());
    if (user_name.is_empty() && user_address.is_empty()) || (field_name.is_empty() && field_address.is_empty()) {
        return Response::text("missing required parameter user_name or field_name").with_status_code(400);
    }

    let user = default_global_db().select_user(Some(user_name), Some(user_address));
    if user.is_none() {
        return Response::text("user not found").with_status_code(404);
    }

    let field = default_global_db().select_field(Some(field_name), Some(field_address));
    if field.is_err() {
        return Response::text("field not found").with_status_code(404);
    }

    let score = default_global_db().select_score(&field.unwrap().address, &user.unwrap().address);

    Response::text(score.score.to_string())
}

fn create_user(request: &Request) -> Response {
    let user_name = request.get_param("user_name").unwrap_or("".to_string());
    let user_address = address(request).unwrap();
    
    if user_name.is_empty() {
        return Response::text("user_name should not be empty").with_status_code(400);
    }

    let user = User::new(user_address, user_name);
    match user.persist() {
        Ok(_) => Response::text("user created"),
        Err(e) => Response::text(e).with_status_code(400),
    }
}

fn post(request: &Request) -> Response {
    let from = address(request).unwrap();

    let field = match default_global_db().select_field(request.get_param("field_name"), request.get_param("field_address")) {
        Ok(value) => value,
        Err(_) => return Response::text("field not found").with_status_code(404),
    };

    let title = match request.get_param("title") {
        Some(value) => value,
        None => return Response::text("missing required parameter title").with_status_code(400),
    };

    let content = match request.get_param("content") {
        Some(value) => value,
        None => return Response::text("missing required parameter content").with_status_code(400),
    };

    let post = Post::new(from, field.address, title, content);
    match post.persist() {
        Ok(_) => Response::text("post created"),
        Err(detail) => Response::text(detail).with_status_code(400),
    }
}

fn comment(request: &Request) -> Response {
    let address = address(request).unwrap();

    let content = match request.get_param("content") {
        Some(value) => value,
        None => return Response::text("missing required parameter content").with_status_code(400),
    };

    let to = match request.get_param("to") {
        Some(value) => value,
        None => return Response::text("missing required parameter to").with_status_code(400),
    };

    let field_address = match request.get_param("field_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter field_address").with_status_code(400),
    };

    match Comment::new(address, to, content, field_address).persist() {
        Ok(_) => Response::text("comment created"),
        Err(detail) => Response::text(detail).with_status_code(400),
    }
}

fn filter_post(request: &Request) -> Response {
    let field_name = request.get_param("field_name");
    let field_address = request.get_param("field_address");

    let field = match default_global_db().select_field(field_name, field_address) {
        Ok(value) => value,
        Err(_) => return Response::text("field not found").with_status_code(404),
    };

    let level = request.get_param("level").map(|l| l.parse::<u8>().unwrap_or(0));
    let keyword = request.get_param("keyword");
    let ordering_str = request.get_param("ordering").unwrap_or("timestamp".to_string());
    let ascending_str = request.get_param("ascending").unwrap_or("false".to_string());
    let max_results_str = request.get_param("max_results").unwrap_or("10".to_string());

    let ordering = match ordering_str.to_lowercase().as_str() {
        "score" => Ordering::ByScore,
        "upvote" => Ordering::ByUpVote,
        "downvote" => Ordering::ByDownVote,
        "upvote-downvote" => Ordering::ByUpvoteSubDownVote,
        _ => Ordering::ByTimestamp,
    };

    let ascending = ascending_str.to_lowercase() == "true";
    let max_results = max_results_str.parse::<u32>().unwrap_or(10);

    let option = FilterOption {
        level,
        keyword,
        ordering,
        ascending,
        max_results,
    };

    match field.filter_posts(option) {
        Ok(posts) => {
            match serde_json::to_string(&posts) {
                Ok(json) => Response::text(json),
                Err(_) => Response::text("failed to serialize posts").with_status_code(500),
            }
        }
        Err(e) => Response::text(e).with_status_code(400),
    }
}

fn upvote(request: &Request) -> Response {
    let address = address(request).unwrap();

    let target_address = match request.get_param("target_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter target_address").with_status_code(400),
    };

    let field_address = match request.get_param("field_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter field_address").with_status_code(400),
    };

    // Try to handle post upvote first
    match default_global_db().select_post(&target_address) {
        Ok(mut post) => {
            match post.upvote(&address) {
                Ok(_) => return Response::text("post upvoted successfully"),
                Err(e) => return Response::text(e).with_status_code(400),
            }
        },
        Err(_) => {
            // If not a post, try to handle as comment
            match Comment::from_db(target_address) {
                Ok(mut comment) => {
                    match comment.upvote(&address) {
                        Ok(_) => return Response::text("comment upvoted successfully"),
                        Err(e) => return Response::text(e).with_status_code(400),
                    }
                },
                Err(_) => return Response::text("target not found").with_status_code(404),
            }
        }
    }
}

fn downvote(request: &Request) -> Response {
    let address = address(request).unwrap();

    let target_address = match request.get_param("target_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter target_address").with_status_code(400),
    };

    let field_address = match request.get_param("field_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter field_address").with_status_code(400),
    };

    // Try to handle post downvote first
    match default_global_db().select_post(&target_address) {
        Ok(mut post) => {
            match post.downvote(&address) {
                Ok(_) => return Response::text("post downvoted successfully"),
                Err(e) => return Response::text(e).with_status_code(400),
            }
        },
        Err(_) => {
            // If not a post, try to handle as comment
            match Comment::from_db(target_address) {
                Ok(mut comment) => {
                    match comment.downvote(&address) {
                        Ok(_) => return Response::text("comment downvoted successfully"),
                        Err(e) => return Response::text(e).with_status_code(400),
                    }
                },
                Err(_) => return Response::text("target not found").with_status_code(404),
            }
        }
    }
}

fn login(request: &Request) -> Response {
    let body = input::plain_text_body(request).unwrap();
    let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let pubkey = match json_body.get("pubkey") {
        Some(pubkey) => pubkey.as_str().unwrap(),
        None => return Response::text("pubkey field is needed in http body").with_status_code(400),
    };
    let signed_pubkey = match json_body.get("signed_pubkey") {
        Some(signed_pubkey) => signed_pubkey.as_str().unwrap(),
        None => return Response::text("signed_pubkey field is needed in http body").with_status_code(400),
    };

    let pubkey_bytes = BASE64_STANDARD.decode(pubkey).unwrap();
    let signed_pubkey_bytes = BASE64_STANDARD.decode(signed_pubkey).unwrap();

    match verify_signature(&pubkey_bytes, &signed_pubkey_bytes, &pubkey_bytes) {
        true => {
            // Generate a unique session ID
            let sid = generate_unique_address();
            
            // Store session information
            let mut sessions_storage = GLOBAL_SESSION_STORGE.lock().unwrap();
            sessions_storage.insert(sid.clone(), SessionStorage {
                logined: true,
                address: pubkey.to_string(),
            });
            
            Response::text(format!("login successful, SID={}", sid))
        },
        false => {
            Response::text("cannot verify signature, please encrypt your address with secret key").with_status_code(401)
        }
    }
}

fn user_rename(request: &Request) -> Response {
    match (request.get_param("name"), request.get_param("address")) {
        (Some(name), Some(address)) => match User::new(address, name).persist() {
            Ok(_) => Response::text("user renamed"),
            Err(detail) => Response::text(detail).with_status_code(400),
        },
        _ => Response::text("missing required parameter name or address").with_status_code(400),
    }
}
