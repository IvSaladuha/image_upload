use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::future::{ok, Future};


fn index(_req: HttpRequest) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("text/html").body("Hello!"),
    ))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[test]
    fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "multipart/form-data").to_http_request();

        let resp = test::block_on(index(req)).unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK)
    }
}
