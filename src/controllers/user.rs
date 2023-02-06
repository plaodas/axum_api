use axum::{
    http::StatusCode,
    Json, 
};
use serde_json::json;


use crate::{error::AppError, models::auth::Claims};

pub async fn user_profile(claims:Claims) -> Result<axum::Json<serde_json::Value>, AppError>{

    // if the token is verified and data is extracted from the token by the
    // implemention in utils.rs then only the below code will run
    Ok(Json(json!({"code":StatusCode::OK.as_str(), "email": &claims.email})))
}