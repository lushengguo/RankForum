extern crate rankforum;

use rankforum::db;
use rankforum::field;
use rankforum::post;
use rankforum::score;
use rankforum::user;

use env_logger;
use rouille::*;

fn main() {
    env_logger::init();

    rouille::start_server("localhost:8000", move |request| {
        rouille::log(request, std::io::stdout(), || {
            router!(request,
                (GET) (/) => {
                    // When viewing the home page, we return an HTML document described below.
                    rouille::Response::html(FORM)
                },
                (GET) (/posts) => {
                    rouille::Response::html(FORM)
                },
                (POST) (/create_field) => {
                    db::DB::create_field(&request);

                    rouille::Response::html("Success! <a href=\"/\">Go back</a>.")
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}

// The HTML document of the home page.
static FORM: &str = r#"
<html>
    <head>
        <title>Form</title>
    </head>
    <body>
        <form action="submit" method="POST" enctype="multipart/form-data">
            <p><input type="text" name="txt" placeholder="Some text" /></p>

            <p><input type="file" name="files" multiple /></p>

            <p><button>Upload</button></p>
        </form>
    </body>
</html>
"#;
