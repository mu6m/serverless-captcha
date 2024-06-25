use std::env;
use sha2::{ Sha256, Digest };
use jsonwebtoken::{
    encode,
    decode,
    Header,
    EncodingKey,
    DecodingKey,
    Validation,
    Algorithm,
    TokenData,
};
use serde::{ Deserialize, Serialize };

// Define the claims struct for the JWT payload
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

pub fn generate_token(code: &str) -> String {
    // Hash the input code using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&code);
    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);

    // Calculate expiration time (5 minutes from now)
    let now = chrono::Utc::now();
    let expiration = now.timestamp() + 5 * 60; // 5 minutes in seconds

    let claims = Claims {
        sub: hash_hex.clone(),
        exp: expiration,
    };
    let enc_key = env::var("JWT_SECRET").unwrap();
    match encode(&Header::default(), &claims, &EncodingKey::from_secret(enc_key)) {
        Ok(token) => token,
        Err(e) => format!("Error generating token: {}", e),
    }
}

pub fn check_token(token: &str, code: &str) -> bool {
    // Set up decoding key and validation
    let enc_key = env::var("JWT_SECRET").unwrap();
    let decoding_key = DecodingKey::from_secret(enc_key);
    let validation = Validation::new(Algorithm::HS256);

    let mut hasher = Sha256::new();
    hasher.update(&code);
    let hash_result = hasher.finalize();
    let hash_hex = format!("{:x}", hash_result);
    // Decode the JWT
    match decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(TokenData { claims, .. }) => {
            // Compare the `sub` claim with the hashed code
            claims.sub == hash_hex
        }
        Err(err) => {
            println!("Error decoding JWT: {}", err);
            false // Return false if there's any error
        }
    }
}
