use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()).await {
        Ok(_res) => Ok(req),
        Err(_err) => Err((ErrorUnauthorized(AuthenticationError::from(config)), req)),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: usize,
}

async fn validate_token(token: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let authority = env::var("AUTHORITY").expect("AUTHORITY must be set");
    let audience = env::var("AUDIENCE").expect("AUDIENCE must be set");
    let jwks = fetch_jwks(&format!("{}{}", authority, ".well-known/jwks.json"))
        .await
        .expect("failed to fetch jwks");
    let header = decode_header(token)?;
    let kid = match header.kid {
        Some(k) => k,
        None => return Err("Token doesn't have a `kid` header field".into()),
    };
    let decoded_token: TokenData<Claims>;
    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                let mut validation = Validation::new(j.common.algorithm.unwrap());
                validation.set_audience(&[audience]);
                validation.validate_exp = false;
                decoded_token = decode::<Claims>(token, &decoding_key, &validation).unwrap();
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        return Err("No matching JWK found for the given kid".into());
    }
    Ok(decoded_token)
}

async fn fetch_jwks(uri: &str) -> Result<jwk::JwkSet, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json().await?;
    return Ok(val);
}
