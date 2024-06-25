extern crate captcha;

use my_api::generate_token;

use captcha::filters::{ Noise, Wave };
use captcha::{ Captcha };

use serde_json::json;
use vercel_runtime::{ run, Body, Error, Request, Response, StatusCode };

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let mut c = Captcha::new();
    c.add_chars(5).apply_filter(Noise::new(0.2)).apply_filter(Wave::new(2.0, 20.0)).view(220, 120);
    let image = c.as_base64().expect("Error.");
    let token = generate_token(c.chars_as_string().as_str());
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(
                json!({
              "image": image,
              "token": token,
            })
                    .to_string()
                    .into()
            )?
    )
}
