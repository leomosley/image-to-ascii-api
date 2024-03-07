use std::path::PathBuf;
use rocket::{get, routes};
use generate::{generate, Params};

mod generate;
mod convert;
mod font;
mod gif;
mod progress;
mod metrics;

#[get("/")]
fn index() -> &'static str {
    "image-to-acsii-api"
}

#[get("/<image_url..>")]
fn get_image_url(image_url: PathBuf) -> String {
    let string_url = match image_url.to_str() {
        Some(url) => url,
        None => {
            return String::from("Invalid URL");
        }
    };
    let args = Params {
        image_url: format!("{}{}", "https://", string_url).as_str(),
        font: "bitocra-13",
        alphabet: "alphabet",
        width: 150,
        metric: "grad",
        threads: 1,
        no_color: false,
        brightness_offset: 0.0,
        noise_scale: 0.0,
        out_path: None,
        fps: 30.0,
        no_edge_detection: false,
    };

    format!("URL: {}", string_url)
}

#[tokio::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, get_image_url])
        .launch()
        .await
        .expect("Rocket failed to launch");
}
