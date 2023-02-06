//cookieからaccess_tokenを取得
//復号してClaimsを戻す

use axum::{
    async_trait,
    extract::{FromRequest,RequestParts,TypedHeader},
};
use jsonwebtoken::{decode, Validation, TokenData};
use headers::Cookie;

use crate::{
    error::AppError, 
    models::auth::Claims,
    utils::time, 
    KEYS
};


#[async_trait]
impl<B> FromRequest<B> for Claims
where B: Send,
{
    type Rejection = AppError;

    async fn from_request(req:&mut RequestParts<B> )->Result<Self, Self::Rejection>{
        
        // get tokens from request header
        let TypedHeader(cookie) = TypedHeader::<Cookie>::from_request(req)
            .await
            .map_err(|_| AppError::InvalidToken)?;

        let access_token = cookie.get("access_token").unwrap_or("").to_string();

        tracing::debug!("tokens: {:?}", &access_token);

        let decrypted_access_token = decrypt_token( &access_token )?;
        tracing::debug!("decrypted_access_token: {:?}", &decrypted_access_token);

        // token expiration check
        if is_token_expired(&decrypted_access_token.claims.exp) {
            // access token is expired 
            tracing::debug!("access token is invalid: {:?}", &decrypted_access_token.claims);
            return Err(AppError::InvalidToken);
        }

        Ok(decrypted_access_token.claims)
    }
}

fn decrypt_token( token: &str ) -> Result<TokenData<Claims>, AppError>{
    return decode::<Claims>(
        token, &KEYS.decoding, &Validation::default()
    )
    .map_err(|_| AppError::InvalidToken);
}

fn is_token_expired(exp:&u64) -> bool{
    return time::current_secs_since_epoch() > *exp;
}