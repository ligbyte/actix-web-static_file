use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

async fn health_check() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/static", "F:/apk").show_files_listing())
            .service(web::scope("").route("/health", web::get().to(health_check)))
            .service(web::scope("/app").route("/hello", web::get().to(index)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(
            App::new().service(web::scope("/app").route("/hello", web::get().to(index))),
        )
        .await;
        let req = test::TestRequest::with_uri("/app/hello").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
