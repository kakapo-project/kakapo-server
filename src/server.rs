use std::path::PathBuf;
use std::path::Path;

use actix::prelude::*;
use actix;
use actix_web::fs;
use actix_web::middleware::Logger;
use actix_web::http;
use actix_web::middleware::cors::Cors;


use openssl::ssl::SslAcceptor;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use actix_web::App;

use AppStateBuilder;
use AppState;

use view::extensions::ProcedureExt;

pub struct Server {
    system: actix::SystemRunner,
    host: String,
    port: u16,
    frontend_path: Option<PathBuf>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            system: actix::System::new("Kakapo"),
            host: "127.0.0.1".to_string(),
            port: 1845,
            frontend_path: None,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn frontend_path(mut self, frontend_path: &Path) -> Self {
        self.frontend_path = Some(frontend_path.to_path_buf());
        self
    }

    pub fn run(self, state_builder: AppStateBuilder) -> i32 {

        let server_addr = (&self.host[..], self.port);
        let is_secure = false;

        let state = state_builder.done();

        let frontend_path = self.frontend_path;

        let mut server_cfg = actix_web::server::new(move || {

            let app = App::with_state(state.clone())
                .middleware(Logger::new("Responded [%s] %b bytes %Dms"))
                .middleware(Logger::new(r#"Requested [%r] FROM %a "%{User-Agent}i""#))
                .configure(move |app| {
                    Cors::for_app(app)
                        .allowed_origin("http://localhost:3000")
                        .allowed_origin("http://localhost:1845")
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600)
                        .add_routes()
                        .register()
                });

            if let Some(ref static_files) = frontend_path {

                let mut index_file = static_files.to_owned();
                index_file.push("index.html");

                app.resource("/", |r| {
                        r.method(http::Method::GET).f(move |req| {
                            fs::NamedFile::open(index_file.to_owned())
                        })
                    })
                    .handler(
                    "/",
                    fs::StaticFiles::new(&static_files)
                        .unwrap()
                        .show_files_listing())
            } else {
                app
            }

        });

        server_cfg = server_cfg
            .workers(num_cpus::get())
            .server_hostname("www.kakapo.ai".to_string())
            .keep_alive(30);


        debug!("is_secure: {:?}", is_secure);
        let http_server = /* if is_secure {
            let ssl_cert_privkey_path = Env::ssl_cert_privkey_path();
            let ssl_cert_fullchain_path = Env::ssl_cert_fullchain_path();

            let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            ssl_builder
                .set_private_key_file(ssl_cert_privkey_path, SslFiletype::PEM)
                .unwrap();
            ssl_builder.set_certificate_chain_file(ssl_cert_fullchain_path).unwrap();


            server_cfg
                .bind_ssl(server_addr, ssl_builder)
                .unwrap()

        } else {
        */
            server_cfg
                .bind(server_addr)
                .unwrap();
        /*
        };
        */

        http_server
            .shutdown_timeout(30)
            .start();

        info!("Kakapo server started on \"{:?}\"", server_addr);

        self.system.run()
    }
}
