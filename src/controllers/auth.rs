use axum::{
    Extension, 
    http::StatusCode,
    Json, 
    response::IntoResponse
};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Value};
use sqlx::PgPool;
use headers::{HeaderMap, HeaderValue};

use crate::{
    error::AppError,
    models::{self, auth::Claims},
    utils::{time::timestamp_secs_from_now, password},
    KEYS,
};


// login process
pub async fn login(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) ->  Result<impl IntoResponse, impl IntoResponse> {

    // check if email or password is a blank string
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential)
    }

    // get the user for the email from database
    let user = sqlx::query_as::<_,models::auth::User>(r#"
SELECT email, password FROM users WHERE email = $1
"#,)
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err|{
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(user) = user {
    // if a user  exists :
        
        let verified = password::verify_password_hash( credentials.password, user.password );
        if let Err(_) = verified {
            Err(AppError::WrongCredential)
        }else{

            // create access_tokens
            let access_token = create_access_token(&credentials.email, 60 * 60 * 8)?;

            let mut headers = HeaderMap::new();
            headers.insert(
                axum::http::header::SET_COOKIE, 
                HeaderValue::from_str( format!("access_token={}; path=/; SameSite=Strict; HttpOnly", access_token).as_str() ).unwrap()
            );

            Ok((headers, Json(json!( {"code":StatusCode::OK.as_str() ,"data":access_token}))))
        }
    }else{
        // if a user does not exist
        Err(AppError::UserDoesNotExist)
    }

}

// 
fn create_access_token(email: &String, duration_secs: u64)->Result<String, AppError>{
    return encode(
        &Header::default(), 
        &Claims {
            email: email.to_owned(),
            exp: timestamp_secs_from_now(duration_secs),
        }, 
        &KEYS.encoding
    ).map_err(|_|AppError::TokenCreation);
}



pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>,AppError>{
    // check if email or password is a blank string
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredential)
    }

    // get the user for the email from database
    let user = sqlx::query_as::<_,models::auth::User>(r#"
SELECT email, password FROM users WHERE email = $1
"#,)
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err|{
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = user {
        // if a user with email already exists send error
        return  Err(AppError::UserAlreadyExist)
    }

    // hash password
    let hashed_password = password::compute_password_hash(&credentials.password)
        .map_err(|_|{
            AppError::InternalServerError
        })?;

    tracing::debug!("hashed_password:{}",&hashed_password);

    let result = sqlx::query("
INSERT INTO users(email, password) VALUES($1, $2)
")
    .bind(&credentials.email)
    .bind(&hashed_password)
    .execute(&pool)
    .await
    .map_err(|_|{
        AppError::InternalServerError
    })?;
    
    if result.rows_affected() < 1{
        Err(AppError::InternalServerError)
    }else {
        Ok(Json(json!( {"code":StatusCode::CREATED.as_str() ,"msg":"registered successfully"})))
    }

}

