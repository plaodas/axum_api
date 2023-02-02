use crate::{error::AppError, models::auth::Claims};

pub async fn user_profile(claims: Claims) -> Result<axum::Json<serde_json::Value>, AppError>{
    // if the token is verified and data is extracted from the token by the
    // implemention in utils.rs then only the below code will run
    Ok(axum::Json(serde_json::json!({"email": claims.email})))
}