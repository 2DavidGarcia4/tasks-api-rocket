use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::serde::{json::Json, {Serialize, Deserialize}};
use uuid::Uuid;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation, encode, Header, TokenData, EncodingKey};
use dotenv::dotenv;
use std::env;
use crate::utils::error::ErrorResponse;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    pub id: Uuid,
    pub email: String,
    pub exp: i64
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = Json<ErrorResponse>;
  
    async fn from_request(request: &'r Request<'_>,) -> Outcome<Self, Json<ErrorResponse>> {
        if let Some(header_auth) = request.headers().get_one("Authorization") {
            let principal = &header_auth[0..3];
            if principal.to_lowercase() == "jwt" {
                let token = header_auth[3..header_auth.len()].trim();
                return match decode_token(token.to_owned()) {
                    Ok(decoded_token) => {
                        let data_t = decoded_token.claims;
                        Outcome::Success(data_t)
                    },
                    Err(err) => Outcome::Failure((
                        Status::BadRequest,
                        Json(ErrorResponse {message: err})
                    ))
                } 
            }
            return Outcome::Failure((Status::BadRequest, Json(ErrorResponse {
                message: "Error".to_owned()
            })));
        } else {
            return Outcome::Failure((Status::BadRequest, Json(ErrorResponse {
                message: "Error".to_owned()
            })));
        }
    }
}
 

pub fn encode_token(claims: UserToken) -> String {
    dotenv().ok();
    let secret = env::var("ESPECIAL_WORD").expect("Error getting special word");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_str().as_ref())).unwrap()
}

pub fn decode_token(token: String) -> Result<TokenData<UserToken>, String> {
    dotenv().ok();
    let secret = env::var("ESPECIAL_WORD").expect("Error getting special word");
    let key = DecodingKey::from_secret(secret.as_str().as_ref());
    match decode::<UserToken>(&token, &key, &Validation::default()) {
        Ok(token_data) => {
            if token_data.claims.exp > Utc::now().timestamp() {
                Ok(token_data)
            }   else {
                Err("Expired token".to_owned())
            }
        },
        Err(_) => Err("Invalid token".to_owned())
    }
}

// fn verify_token(token_data: &TokenData<UserToken>, conn: &PgConnection) -> bool {
//     User::is_valid_login_session(&token_data.claims, conn)
// }
