use actix_web::{App, HttpServer, web};

mod error;
mod handler;
mod utils;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:7878";
    println!("Listening at address http://{}", address);

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::get().to(handler::read_file))
                    .route(web::post().to_async(handler::create_file))
                    .route(web::put().to_async(handler::update_file))
                    .route(web::delete().to(handler::delete_file))
            )
            .default_service(web::route().to(handler::not_found))
    })
    .bind(address)?
    .run()
}