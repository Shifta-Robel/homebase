use actix_web::{
    App, HttpServer, web,
    // middleware::Logger,
};
use actix_cors::Cors;
use backend::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    //
    // let config: &'static backend::config::AppConfig = &backend::config::AppConfig::build().unwrap();
    let config = backend::config::AppConfig::build().unwrap();
    
    HttpServer::new(|| {
        // let logger = Logger::default();
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            // .wrap(logger)
            .service(api::history::history)
            .service(api::bookmarks::bookmarks)
            .service(api::quickmarks::quickmarks)
            // .service(api::quicklinks::quicklink)
            .service(
                web::resource("/quicklink")
                    .app_data(web::Data::new(backend::config::AppConfig::build().unwrap()))
                    .route(web::get().to(api::quicklinks::quicklink))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
