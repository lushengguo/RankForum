extern crate rankforum;

use rankforum::db;
use rankforum::field;
use rankforum::post;
use rankforum::score;
use rankforum::user;

use env_logger;
use rouille::*;
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init();

    rouille::start_server("localhost:8000", move |request| {
        rouille::log(request, std::io::stdout(), || {
            router!(request,
                (GET) (/) => {
                let path = Path::new("static/index.html");
                    if path.exists() {
                        match File::open(path) {
                            Ok(file) => rouille::Response::from_file("text/html", file),
                            Err(_) => rouille::Response::empty_404(),
                        }
                    } else {
                        rouille::Response::empty_404()
                    }
                },
                (GET) (/user_in_field) => {
                    let user_name = request.get_param("user_name").unwrap_or("".to_string());
                    let user_address = request.get_param("user_address").unwrap_or("".to_string());
                    let field_name = request.get_param("field_name").unwrap_or("".to_string());
                    let field_address = request.get_param("field_address").unwrap_or("".to_string());
                    if (user_name == "" && user_address == "") || (field_name == "" && field_address == "") {
                        return rouille::Response::empty_400()
                    }
                    rouille::Response::html("")
                },
                (GET) (/posts) => {
                    rouille::Response::html("")
                },
                (POST) (/create_field) => {
                match(request.get_param("name"), request.get_param("address")) {
                    (Some(name), Some(address)) => {
                        let field = field::Field::new(name, address);
                        field.save_to_db();
                        return rouille::Response::html("")
                    },
                    _ => return rouille::Response::empty_400()
                }
                    // let name = try_or_400!(request.get_param("name"), "Missing field_name parameter.");
                    // db::DB::save_field(&request);
                    rouille::Response::html("")
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}
