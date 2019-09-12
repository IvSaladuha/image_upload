use image::{DynamicImage, ImageResult};
use actix_http::Error;
use actix_rt::System;
use futures::{future::lazy, Future};

fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "actix_http=trace");
    env_logger::init();

    let result: Result<DynamicImage, Error> = System::new("test").block_on(lazy(|| {
        awc::Client::new()
            .get("https://images-na.ssl-images-amazon.com/images/I/81ExUiupQrL._SL1500_.jpg") // <- Create request builder
            .header("User-Agent", "Actix-web")
            .send() // <- Send http request
            .from_err()
            .and_then(|mut response| {
                // <- server http response
                println!("Response: {:?}", response);

                // read response body
                response
                    .body()
                    .from_err()
                    .map(|body| image::load_from_memory(&body).unwrap())
            })
    }));
    match result {
        Result::Ok(img) => {
            println!("Got image");
            let new_img = img.resize_exact(100, 100, image::FilterType::Triangle);
            new_img.save("out.png");
        },
        Result::Err(err) => {
            println!("Got error")
        }
    };
    Result::Ok(())
}
