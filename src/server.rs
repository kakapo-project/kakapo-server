
use actix::prelude::*;
use actix_web::middleware::Logger;
use actix_web::http;
use actix_web::middleware::cors::Cors;

use openssl::ssl::SslAcceptor;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;
use actix_web::App;
use AppStateBuilder;

use view::extensions::ProcedureExt;
use view::route_builder::RouteBuilder;

pub fn serve() {
    let server_addr = "127.0.0.1:8080";
    let is_secure = false;

    let mut server_cfg = actix_web::server::new(move || {

        let secret = "Hello World";
        let mut state = AppStateBuilder::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .num_threads(1)
            .done();

        let route_builder = RouteBuilder::build(&mut state);

        App::with_state(state)
            .middleware(Logger::new("Responded [%s] %b bytes %Dms"))
            .middleware(Logger::new(r#"Requested [%r] FROM %a "%{User-Agent}i""#))
            .configure(move |app| Cors::for_app(app)
                //.allowed_origin("http://localhost:3000") //TODO: this doesn't work in the current version of cors middleware https://github.com/actix/actix-web/issues/603
                //.allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .add_routes(&route_builder)
                .register())
    });

    server_cfg = server_cfg
        .workers(num_cpus::get())
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
            .bind_ssl(&server_addr, ssl_builder)
            .unwrap()

    } else {
        */
        server_cfg
            .bind(&server_addr)
            .unwrap();
    /*
    };
    */

    http_server
        .shutdown_timeout(30)
        .start();

    info!("Kakapo server started on \"{}\"", &server_addr);
}