use my_api::check_token;
use serde_json::json;
use vercel_runtime::{ run, Body, Error, Request, Response, StatusCode };

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() == "POST" {
        match serde_json::from_slice::<serde_json::Value>(req.body().as_ref()) {
            Ok(json_body) => {
                let token = match json_body["token"].as_str() {
                    Some(token) => token,
                    None => {
                        return create_error_response(
                            StatusCode::BAD_REQUEST,
                            "Token key is missing or invalid"
                        );
                    }
                };

                let code = match json_body["code"].as_str() {
                    Some(code) => code,
                    None => {
                        return create_error_response(
                            StatusCode::BAD_REQUEST,
                            "Code key is missing or invalid"
                        );
                    }
                };

                let is_token = check_token(token, code);
                Ok(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(
                            json!({
                                "check": is_token,
                            })
                                .to_string()
                                .into()
                        )?
                )
            }
            Err(_) => create_error_response(StatusCode::BAD_REQUEST, "Invalid JSON"),
        }
    } else {
        create_error_response(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
    }
}

fn create_error_response(status: StatusCode, message: &str) -> Result<Response<Body>, Error> {
    Ok(
        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "error": message,
                }).to_string().into()
            )?
    )
}
