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
use log::{info, warn, error, debug};

lazy_static! {
    static ref GLOBAL_SESSION_STORGE: Mutex<HashMap<String, SessionStorage>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct SessionStorage {
    logined: bool,
    address: Address,
}

// Add CORS headers helper function
fn add_cors_headers(response: Response) -> Response {
    debug!("Adding CORS headers");
    response.with_additional_header("Access-Control-Allow-Origin", "*")
           .with_additional_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
           .with_additional_header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, SID")
           .with_additional_header("Access-Control-Max-Age", "86400")
}

pub fn handle_route(request: &Request) -> Response {
    debug!("Processing request: {} {}", request.method(), request.url());
    
    // Handle preflight requests
    if request.method() == "OPTIONS" {
        info!("Received CORS preflight request");
        return add_cors_headers(Response::empty_204());
    }
    
    // Check user login
    if request.url() != "login" && request.method() == "POST" && !user_already_logined(request) {
        warn!("Unauthorized user attempted to access protected endpoint");
        return add_cors_headers(rouille::Response::text("please login first").with_status_code(401));
    }

    // Build normal response
    let response = router!(request,
        (POST) (/login) => {
            info!("Received login request");
            login(request)
        },
        (POST) (/post) => {
            info!("Received post creation request");
            post(request)
        },
        (POST) (/comment) => {
            info!("Received comment request");
            comment(request)
        },
        (GET) (/filter_post) => {
            debug!("Filtering posts");
            filter_post(request)
        },
        (POST) (/rename_user) => {
            info!("Received rename request");
            user_rename(request)
        },
        (POST) (/create_user) => {
            info!("Received user creation request");
            create_user(request)
        },
        (POST) (/upvote) => {
            debug!("Received upvote request");
            upvote(request)
        },
        (POST) (/downvote) => {
            debug!("Received downvote request");
            downvote(request)
        },
        (GET) (/query_user_address) => {
            debug!("Querying user address");
            query_user_address(request)
        },
        (GET) (/query_field_address) => {
            debug!("Querying field address");
            query_field_address(request)
        },
        (GET) (/query_score_in_field) => {
            debug!("Querying score in field");
            query_score_in_field(request)
        },
        (POST) (/create_field) => {
            info!("Creating new field");
            create_field(request)
        },
        (GET) (/get_all_fields) => {
            debug!("Getting all fields");
            get_all_fields(request)
        },
        (GET) (/get_field_posts) => {
            debug!("Getting field posts");
            get_field_posts(request)
        },
        (GET) (/user_info) => {
            debug!("Getting user info");
            get_user_info(request)
        },
        (GET) (/user_posts) => {
            debug!("Getting user posts");
            get_user_posts(request)
        },
        _ => {
            warn!("Unknown route: {} {}", request.method(), request.url());
            rouille::Response::empty_404()
        }
    );
    
    // Add CORS headers to all responses
    add_cors_headers(response)
}

fn get_session_cache(request: &Request) -> Option<SessionStorage> {
    let sid = match request.get_param("SID") {
        Some(sid) => sid,
        None => {
            debug!("Request has no session ID");
            return None;
        },
    };

    let sessions_storage = GLOBAL_SESSION_STORGE.lock().unwrap();
    match sessions_storage.get(&sid) {
        Some(cache) => {
            debug!("Found session: {}", sid);
            Some(cache.clone())
        },
        None => {
            debug!("Session does not exist: {}", sid);
            None
        }
    }
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
    if let Some(post_address) = request.get_param("post_address") {
        match default_global_db().select_post(&post_address) {
            Ok(post) => {
                match serde_json::to_string(&vec![post]) {
                    Ok(json) => return Response::text(json)
                        .with_additional_header("Content-Type", "application/json"),
                    Err(_) => return Response::text("failed to serialize post data").with_status_code(500),
                }
            }
            Err(_) => return Response::text("post not found").with_status_code(404),
        }
    }

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
                Ok(json) => Response::text(json)
                    .with_additional_header("Content-Type", "application/json"),
                Err(_) => Response::text("failed to serialize posts").with_status_code(500),
            }
        }
        Err(e) => Response::text(e).with_status_code(400),
    }
}

fn upvote(request: &Request) -> Response {
    let address = match address(request) {
        Some(addr) => addr,
        None => {
            warn!("Unauthorized upvote request");
            return Response::text("Unauthorized operation").with_status_code(401);
        }
    };

    let target_address = match request.get_param("target_address") {
        Some(value) => value,
        None => {
            warn!("Upvote request missing target_address");
            return Response::text("missing required parameter target_address").with_status_code(400);
        },
    };

    debug!("User {} attempting to upvote {}", address, target_address);
    
    match default_global_db().select_post(&target_address) {
        Ok(mut post) => {
            match post.upvote(&address) {
                Ok(_) => return Response::text("post upvoted successfully"),
                Err(e) => return Response::text(e).with_status_code(400),
            }
        },
        Err(_) => {
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
    let address = match address(request) {
        Some(addr) => addr,
        None => {
            warn!("Unauthorized downvote request");
            return Response::text("Unauthorized operation").with_status_code(401);
        }
    };

    let target_address = match request.get_param("target_address") {
        Some(value) => value,
        None => {
            warn!("Downvote request missing target_address");
            return Response::text("missing required parameter target_address").with_status_code(400);
        },
    };

    debug!("User {} attempting to downvote {}", address, target_address);
    
    match default_global_db().select_post(&target_address) {
        Ok(mut post) => {
            match post.downvote(&address) {
                Ok(_) => return Response::text("post downvoted successfully"),
                Err(e) => return Response::text(e).with_status_code(400),
            }
        },
        Err(_) => {
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
    let body = match input::plain_text_body(request) {
        Ok(body) => body,
        Err(e) => {
            error!("Failed to read login request body: {:?}", e);
            return Response::text("Unable to read request body").with_status_code(400);
        },
    };
    
    let json_body: serde_json::Value = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to parse login request JSON: {:?}", e);
            return Response::text("Request body must be valid JSON").with_status_code(400);
        },
    };
    
    let pubkey = match json_body.get("pubkey") {
        Some(pubkey) => match pubkey.as_str() {
            Some(str) => str,
            None => return Response::text("pubkey must be a string").with_status_code(400),
        },
        None => return Response::text("HTTP request body must contain pubkey field").with_status_code(400),
    };
    
    let signed_pubkey = match json_body.get("signed_pubkey") {
        Some(signed_pubkey) => match signed_pubkey.as_str() {
            Some(str) => str,
            None => return Response::text("signed_pubkey must be a string").with_status_code(400), 
        },
        None => return Response::text("HTTP request body must contain signed_pubkey field").with_status_code(400),
    };

    let pubkey_bytes = match BASE64_STANDARD.decode(pubkey) {
        Ok(bytes) => bytes,
        Err(_) => return Response::text("pubkey must be valid Base64 encoding").with_status_code(400),
    };
    
    let signed_pubkey_bytes = match BASE64_STANDARD.decode(signed_pubkey) {
        Ok(bytes) => bytes,
        Err(_) => return Response::text("signed_pubkey must be valid Base64 encoding").with_status_code(400),
    };

    match verify_signature(&pubkey_bytes, &signed_pubkey_bytes, &pubkey_bytes) {
        true => {
            let sid = generate_unique_address();
            
            let mut sessions_storage = GLOBAL_SESSION_STORGE.lock().unwrap();
            sessions_storage.insert(sid.clone(), SessionStorage {
                logined: true,
                address: pubkey.to_string(),
            });
            
            if default_global_db().select_user(None, Some(pubkey.to_string())).is_none() {
                let default_name = format!("User_{}", &pubkey[0..8]);
                let _ = User::new(pubkey.to_string(), default_name).persist();
            }
            
            Response::text(format!("login successful, SID={}", sid))
        },
        false => {
            Response::text("Unable to verify signature, please encrypt your address with your private key").with_status_code(401)
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

fn create_field(request: &Request) -> Response {
    let address = address(request).unwrap();
    
    let field_name = match request.get_param("field_name") {
        Some(value) => value,
        None => return Response::text("missing required parameter field_name").with_status_code(400),
    };
    
    if field_name.is_empty() {
        return Response::text("field_name should not be empty").with_status_code(400);
    }
    
    let field_address = crate::generate_unique_address();
    let field = Field::new(field_name, field_address);
    
    match field.persist() {
        Ok(_) => Response::text("field created successfully"),
        Err(e) => Response::text(e).with_status_code(400),
    }
}

fn get_all_fields(request: &Request) -> Response {
    let fields = default_global_db().select_all_fields();
    
    match serde_json::to_string(&fields) {
        Ok(json) => Response::text(json)
            .with_additional_header("Content-Type", "application/json"),
        Err(_) => Response::text("failed to serialize fields data").with_status_code(500),
    }
}

fn get_field_posts(request: &Request) -> Response {
    let field_name = request.get_param("field_name");
    let field_address = request.get_param("field_address");
    
    if field_name.is_none() && field_address.is_none() {
        return Response::text("missing required parameter: field_name or field_address").with_status_code(400);
    }
    
    let field = match default_global_db().select_field(field_name, field_address) {
        Ok(value) => value,
        Err(_) => return Response::text("field not found").with_status_code(404),
    };
    
    let option = FilterOption {
        level: None,
        keyword: None,
        ordering: Ordering::ByTimestamp,
        ascending: false,
        max_results: 100,
    };
    
    match field.filter_posts(option) {
        Ok(posts) => {
            match serde_json::to_string(&posts) {
                Ok(json) => Response::text(json)
                    .with_additional_header("Content-Type", "application/json"),
                Err(_) => Response::text("failed to serialize posts").with_status_code(500),
            }
        }
        Err(e) => Response::text(e).with_status_code(400),
    }
}

fn get_user_info(request: &Request) -> Response {
    let user_address = match address(request) {
        Some(addr) => addr,
        None => return Response::text("User not logged in").with_status_code(401),
    };
    
    let user = match default_global_db().select_user(None, Some(user_address.clone())) {
        Some(user) => user,
        None => {
            return Response::text(format!("User does not exist, address: {}", user_address))
                .with_status_code(404);
        }
    };
    
    match serde_json::to_string(&user) {
        Ok(json) => Response::text(json)
            .with_additional_header("Content-Type", "application/json"),
        Err(_) => Response::text("Failed to serialize user data").with_status_code(500),
    }
}

fn get_user_posts(request: &Request) -> Response {
    let user_address = match request.get_param("user_address") {
        Some(addr) => addr,
        None => {
            match address(request) {
                Some(addr) => addr,
                None => return Response::text("No user address provided and not logged in").with_status_code(400),
            }
        }
    };
    
    let fields = default_global_db().select_all_fields();
    let mut all_user_posts: Vec<Post> = Vec::new();
    
    for field in fields {
        let option = FilterOption {
            level: None,
            keyword: None,
            ordering: Ordering::ByTimestamp,
            ascending: false,
            max_results: 1000,
        };
        
        if let Ok(posts) = field.filter_posts(option) {
            let user_posts: Vec<Post> = posts
                .into_iter()
                .filter(|post| post.from == user_address)
                .collect();
            
            all_user_posts.extend(user_posts);
        }
    }
    
    all_user_posts.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    match serde_json::to_string(&all_user_posts) {
        Ok(json) => Response::text(json)
            .with_additional_header("Content-Type", "application/json"),
        Err(_) => Response::text("Failed to serialize posts data").with_status_code(500),
    }
}
