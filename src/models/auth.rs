use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::serde::json::Json;
use chrono::{Utc, Duration};
use crate::models::users::User;
use crate::db::establish_connection;
use crate::utils::{error::ErrorResponse, crypt::verify_password};
use crate::utils::jwt::{encode_token, UserToken};


#[derive(Debug, Deserialize)]
pub struct UserCredentials<'a> {
  pub email: &'a str,
  pub password: &'a str 
}

#[derive(Debug, Serialize)]
pub struct ResponseToken {
  message: String,
  token: String
}

impl ResponseToken {
    pub fn login(credentials: UserCredentials) -> Result<Json<ResponseToken>, (Status, Json<ErrorResponse>)> {
        let conn = &mut establish_connection();
        let users = User::get_all(conn).clone();
        let user = users.iter().find(|u| u.email == credentials.email);

        if user.is_some() {
            match verify_password(&credentials.password, &user.unwrap().password) {
                Ok(is_veryfied) => {
                    println!("the password is: {}", is_veryfied);

                    if is_veryfied {
                        let claims = UserToken { 
                            id: user.unwrap().id, 
                            email: user.unwrap().email.clone(),
                            exp: (Utc::now() + Duration::hours(4)).timestamp()
                        };
    
                        let token_response = ResponseToken {
                            message: String::from("Correct credentials"),
                            token: encode_token(claims)
                        };
            
                        Ok(Json(token_response))

                    } else {
                        Err((Status::BadRequest, Json(ErrorResponse {message: String::from("Wrong password")})))
                    }
                },

                Err(_) => Err((Status::InternalServerError, Json(ErrorResponse {message: String::from("Password match failed")})))
            }
        } else {
            Err((Status::BadRequest, Json(ErrorResponse {message: String::from("Wrong email")})))
        }        
    }
}