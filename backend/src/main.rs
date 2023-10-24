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
    let config = web::Data::new(
        backend::config::AppConfig::build().unwrap()
    );
    
    HttpServer::new(move || {
        // let logger = Logger::default();
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            // .wrap(logger)
            .service(
                web::resource("/history")
                    .app_data(config.clone())
                    .route(web::get().to(api::history::history))
            )
            .service(
                web::resource("/bookmarks")
                    .app_data(config.clone())
                    .route(web::get().to(api::bookmarks::bookmarks))
            )
            .service(
                web::resource("/quickmarks")
                    .app_data(config.clone())
                    .route(web::get().to(api::quickmarks::quickmarks))
            )
            .service(
                web::resource("/quicklink")
                    .app_data(config.clone())
                    .route(web::get().to(api::quicklinks::quicklink))
            )
            .service(
                web::resource("/reading_list")
                    .app_data(config.clone())
                    .route(web::get().to(api::reading_list::reading_list))
            )
            .service(
                web::resource("/background")
                    .app_data(config.clone())
                    .route(web::get().to(api::background::get_background))
            )
            .service(
                web::resource("/run_command")
                    .app_data(config.clone())
                    .route(web::post().to(api::run_command::run_command))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
