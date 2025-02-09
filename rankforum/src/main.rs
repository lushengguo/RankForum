extern crate rankforum;

use env_logger;
use rankforum::service;
use std::io::Write;
use rouille;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}:{}] [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                std::thread::current().name().unwrap_or("unknown"),
                record.args()
            )
        })
        .init();

    rouille::start_server("localhost:8000", move |request| {
        rouille::log(request, std::io::stdout(), || service::handle_route(&request))
    });
}
