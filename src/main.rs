use std::io;

use actix_web::{web, App as ActixApp, HttpServer};
use clap::Parser;
use dav_server::actix::*;
use dav_server::{fakels::FakeLs, localfs::LocalFs, DavConfig, DavHandler};
use log::warn;

mod error;
mod utils;

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:4918";

struct Config {
    cmd_args: Args,
    // calculated valid Authorization header value
    valid_auth: String,
}

struct App {
    config: Config,
}

fn new_app() -> App {
    env_logger::init();

    let args = Args::parse();
    let valid_auth = utils::basic_auth_header(&args.username, &args.password);

    App {
        config: Config {
            cmd_args: args,
            valid_auth,
        },
    }
}

impl App {
    fn dav_server(&self) -> DavHandler {
        let macos = if cfg!(target_os = "macos") {
            true
        } else {
            false
        };

        DavHandler::builder()
            .filesystem(LocalFs::new(
                &self.config.cmd_args.dest_dir,
                false,
                false,
                macos,
            ))
            .locksystem(FakeLs::new())
            .build_handler()
    }

    fn auth(&self, req: &DavRequest) -> bool {
        let auth_header_v = req.request.headers().get("authorization");
        if auth_header_v.is_none() {
            warn!(
                "auth header missing, client ip: {}",
                utils::get_client_ip(&req.request)
            );
            return false;
        }

        if auth_header_v.unwrap() == &self.config.valid_auth {
            return true;
        } else {
            warn!(
                "invalid auth: {:?}, client ip: {}",
                auth_header_v.unwrap(),
                utils::get_client_ip(&req.request),
            );
        }

        false
    }
}

async fn dav_handler(
    req: DavRequest,
    davhandler: web::Data<DavHandler>,
    app: web::Data<App>,
) -> Result<DavResponse, error::DavAuthError> {
    if !app.auth(&req) {
        return Err(error::DavAuthError::DavAuthError);
    }

    let resp: DavResponse;
    if let Some(prefix) = req.prefix() {
        let config = DavConfig::new().strip_prefix(prefix);
        resp = davhandler.handle_with(config, req.request).await.into();
    } else {
        resp = davhandler.handle(req.request).await.into();
    }
    Ok(resp)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // webdav base dir
    #[arg(short, long)]
    dest_dir: String,

    #[arg(short, long, default_value_t = String::from(DEFAULT_LISTEN_ADDR))]
    listen_addr: String,

    // basic auth username
    #[arg(short, long)]
    username: String,

    // basic auth password
    #[arg(short, long)]
    password: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = new_app();

    println!(
        "app listening on {} serving {}",
        &app.config.cmd_args.listen_addr, &app.config.cmd_args.dest_dir
    );

    let listen_addr = app.config.cmd_args.listen_addr.clone();
    let dav_server = app.dav_server();
    let app_data = web::Data::new(app);
    HttpServer::new(move || {
        let dav_server = dav_server.clone();
        let app_data = app_data.clone();

        ActixApp::new()
            .app_data(web::Data::new(dav_server))
            .app_data(app_data.clone())
            .service(web::resource("/{tail:.*}").to(dav_handler))
    })
    .bind(listen_addr)?
    .run()
    .await
}
