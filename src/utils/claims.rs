use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::{error::AppError, models::auth::Claims, KEYS};

// verify token and extract data from it
// whenever you try to extract claims in the handle it will run this code at first
#[async_trait]
impl<B> FromRequest<B> for Claims
where B: Send,
{
    type Rejection = AppError;

    async fn from_request(req:&mut RequestParts<B> )->Result<Self, Self::Rejection>{
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|_| AppError::InvalidToken)?;

        let data = decode::<Claims>(
                bearer.token(), &KEYS.decoding, &Validation::default()
            )
            .map_err(|_| AppError::InvalidToken)?;
        
        tracing::debug!("claims: {:?}", &data.claims);

        Ok(data.claims)            
    }
}
