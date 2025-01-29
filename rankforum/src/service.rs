use crate::db::*;
use crate::field::*;
use crate::post::*;
use crate::score::*;
use crate::user;
use crate::user::*;
use crate::crypto::*;

use rouille::*;

pub fn query_user_address(request: &Request) -> Response {
    let user_name = request.get_param("user_name").unwrap_or("".to_string());
    if user_name.is_empty() {
        return Response::text("missing required parameter user_name").with_status_code(400);
    }

    let user = DB::user(Some(user_name), None);
    if user.is_none() {
        return Response::text("user not found").with_status_code(404);
    }

    Response::text(user.unwrap().address)
}

pub fn query_field_address(request: &Request) -> Response {
    let field_name = request.get_param("field_name").unwrap_or("".to_string());
    if field_name.is_empty() {
        return Response::text("missing required parameter field_name").with_status_code(400);
    }

    let field = DB::field(Some(field_name), None);
    if field.is_none() {
        return Response::text("field not found").with_status_code(404);
    }

    Response::text(field.unwrap().address)
}

pub fn query_score_in_field(request: &Request) -> Response {
    let user_name = request.get_param("user_name").unwrap_or("".to_string());
    let user_address = request.get_param("user_address").unwrap_or("".to_string());
    let field_name = request.get_param("field_name").unwrap_or("".to_string());
    let field_address = request.get_param("field_address").unwrap_or("".to_string());
    if (user_name.is_empty() && user_address.is_empty()) || (field_name.is_empty() && field_address.is_empty()) {
        return Response::text("missing required parameter user_name or field_name").with_status_code(400);
    }

    let user = DB::user(Some(user_name), Some(user_address));
    if user.is_none() {
        return Response::text("user not found").with_status_code(404);
    }

    let field = DB::field(Some(field_name), Some(field_address));
    if field.is_none() {
        return Response::text("field not found").with_status_code(404);
    }

    let score = DB::score(&field.unwrap().address, &user.unwrap().address);
    if score.is_none() {
        return Response::text("score not found").with_status_code(404);
    }

    Response::text(score.unwrap().to_string())
}

pub fn create_user(request: &Request) -> Response {
    // let user_name = request.get_param("user_name").unwrap_or("".to_string());
    // let user_address = request.get_param("user_address").unwrap_or("".to_string());
    // if user_name.is_empty() {
    //     return Response::text("user_name should not be empty").with_status_code(400);
    // }

    // let user = User::new(user_name);
    // DB::update_user(&user);

    Response::text("user created")
}

pub fn post(request: &Request) -> Response {
    Response::text("not implemented").with_status_code(501)
}

pub fn comment(request: &Request) -> Response {
    Response::text("not implemented").with_status_code(501)
}

pub fn filter_post(request: &Request) -> Response {
    Response::text("not implemented").with_status_code(501)
}
