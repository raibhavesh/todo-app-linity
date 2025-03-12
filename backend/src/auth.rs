use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts, Request, Response,header},
    middleware::Next,
    body::BoxBody,  // Add this import
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::env;

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env")
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (username)
    pub exp: usize,  // expiration timestamp
}

// Create JWT valid for 1 hour
pub fn create_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

// Verify JWT token
pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
}

// Auth middleware function
pub async fn require_auth<B>(
    req: Request<B>, 
    next: Next<B>
) -> Result<Response<BoxBody>, StatusCode>  // Use BoxBody as the generic parameter
where
    B: Send,
{
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].trim())
            } else {
                None
            }
        });

    if let Some(token) = auth_header {
        match verify_jwt(token) {
            Ok(_) => {
                let response = next.run(req).await;
                Ok(response)
            },
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// For extracting user info in handlers
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|value| {
                if value.starts_with("Bearer ") {
                    Some(value[7..].trim())
                } else {
                    None
                }
            })
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token_data = verify_jwt(auth_header)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedUser {
            username: token_data.claims.sub,
        })
    }
}