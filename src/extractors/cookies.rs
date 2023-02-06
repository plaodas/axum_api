use axum::{
    async_trait,
    extract::{FromRequest,RequestParts},
};
use axum::extract::TypedHeader;
use headers::Cookie;

use crate::{error::AppError, models::auth::Tokens};

#[async_trait]
impl<B> FromRequest<B> for Tokens
where B: Send,
{
    type Rejection = AppError;

    async fn from_request(req:&mut RequestParts<B> )->Result<Self, Self::Rejection>{
        
        // get cookies from request header
        let TypedHeader(cookie) = TypedHeader::<Cookie>::from_request(req)
            .await
            .map_err(|_| AppError::InvalidToken)?;

        let tokens = Self{
            access_token: cookie.get("access_token").unwrap_or("").to_string(),
            refresh_token: cookie.get("refresh_token").unwrap_or("").to_string(),
        };

        tracing::debug!("tokens: {:?}", &tokens);

        Ok(tokens)
    }
}