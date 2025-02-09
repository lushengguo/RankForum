use crate::crypto::*;
use crate::db::global_db;
use crate::post::*;
use crate::user::*;
use crate::Address;
use base64::prelude::*;
use lazy_static::lazy_static;
use rouille::*;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_SESSION_STORGE: Mutex<HashMap<String, SessionStorage>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct SessionStorage {
    logined: bool,
    address: Address,
}

pub fn handle_route(request: &Request) -> Response {
    if request.url() != "login" && request.method() == "POST" {
        if !user_already_logined(request) {
            return rouille::Response::text("please login first").with_status_code(401);
        }
    }

    router!(request,
        (POST) (/login) => {
            login(&request)
        },
        (POST) (/post) => {
            post(&request)
        },
        (POST) (/comment) => {
            comment(&request)
        },
        (GET) (/filter_post) => {
            filter_post(&request)
        },
        (POST) (/rename) => {
            user_rename(&request)
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
    match sessions_storage.get(&sid) {
        Some(cache) => Some(cache.clone()),
        None => None,
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
        Some(cache) => {
            return cache.logined;
        }
        None => false,
    }
}

fn query_user_address(request: &Request) -> Response {
    let user_name = request.get_param("user_name").unwrap_or("".to_string());
    if user_name.is_empty() {
        return Response::text("missing required parameter user_name").with_status_code(400);
    }

    let user = global_db().user(Some(user_name), None);
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

    let field = global_db().field(Some(field_name), None);
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

    let user = global_db().user(Some(user_name), Some(user_address));
    if user.is_none() {
        return Response::text("user not found").with_status_code(404);
    }

    let field = global_db().field(Some(field_name), Some(field_address));
    if field.is_err() {
        return Response::text("field not found").with_status_code(404);
    }

    let score = global_db().score(&field.unwrap().address, &user.unwrap().address);
    if score.is_none() {
        return Response::text("score not found").with_status_code(404);
    }

    Response::text(score.unwrap().to_string())
}

fn create_user(request: &Request) -> Response {
    // let user_name = request.get_param("user_name").unwrap_or("".to_string());
    // let user_address = request.get_param("user_address").unwrap_or("".to_string());
    // if user_name.is_empty() {
    //     return Response::text("user_name should not be empty").with_status_code(400);
    // }

    // let user = User::new(user_name);
    // global_db().update_user(&user);

    Response::text("user created")
}

fn post(request: &Request) -> Response {
    let from = address(request).unwrap();

    let field = match global_db().field(request.get_param("field_name"), request.get_param("field_address")) {
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

    match Comment::new(address, to, content).persist() {
        Ok(_) => Response::text("comment created"),
        Err(detail) => Response::text(detail).with_status_code(400),
    }
}

// target of upvote/downvote must be comment or post
// and it ultimately affects the score of the post/comment and user
fn upvote(request: &Request) -> Response {
    let address = address(request).unwrap();

    let target_address = match request.get_param("target_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter target_address").with_status_code(400),
    };

    // let post = match global_db().post(Some(target_address)) {
    //     Some(value) => value,
    //     None => return Response::text("post not found").with_status_code(404),
    // };

    // post.upvote(&address);
    Response::text("upvote success")
}

fn downvote(request: &Request) -> Response {
    let address = address(request).unwrap();

    let target_address = match request.get_param("target_address") {
        Some(value) => value,
        None => return Response::text("missing required parameter target_address").with_status_code(400),
    };

    // let post = match global_db().post(Some(target_address), None) {
    //     Some(value) => value,
    //     None => return Response::text("post not found").with_status_code(404),
    // };

    // post.downvote(&address);
    Response::text("downvote success")
}

fn filter_post(request: &Request) -> Response {
    Response::text("not implemented").with_status_code(501)
}

fn login(request: &Request) -> Response {
    let body = input::plain_text_body(&request).unwrap();
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
        true => Response::text("login successful"),
        false => {
            Response::text("cannot verify signature, please encrypt your address with secret key").with_status_code(401)
        }
    }
}

fn user_rename(request: &Request) -> Response {
    match (request.get_param("name"), request.get_param("address")) {
        (Some(name), Some(address)) => match User::new(address, "".to_string()).rename(name) {
            Ok(_) => return Response::text("user renamed"),
            Err(detail) => return Response::text(detail).with_status_code(400),
        },
        _ => return Response::text("missing required parameter name or address").with_status_code(400),
    };
}
