use axum::{
    async_trait,
    extract::{FromRequest,RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::{error::AppError, models::auth::Claims, utils::time, KEYS};

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

        // token expiration check
        if time::current_secs_since_epoch() > data.claims.exp {
            tracing::debug!("token is invalid: {}", bearer.token());
            Err(AppError::InvalidToken)?
        }

        Ok(data.claims)
    }
}




















// use axum::{
//     async_trait,
//     extract::{FromRequest,RequestParts},
//     headers::{authorization::Bearer, Authorization},
//     TypedHeader,
// };
// use headers::{Cookie, HeaderMap, HeaderMapExt, HeaderValue};
// use jsonwebtoken::{decode, Validation};

// use crate::{error::AppError, models::auth::{Tokens, Claims, Security}, utils::time, KEYS};

// // verify token and extract data from it
// // whenever you try to extract claims in the handle it will run this code at first
// #[async_trait]
// impl<B> FromRequest<B> for Security
// where B: Send,
// {
//     type Rejection = AppError;

//     async fn from_request(req:&mut RequestParts<B> )->Result<Self, Self::Rejection>{

//         let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request(req)
//             .await
//             .map_err(|_| AppError::InvalidToken)?;

//         let TypedHeader(cookie) = TypedHeader::<Cookie>::from_request(req)
//             .await
//             .map_err(|_| AppError::InvalidToken)?;

//         let tokens = Tokens{
//             access_token: "access_token".to_string(),
//             refresh_token: "refresh_token".to_string(),
//         };

//         let data = decode::<Claims>(
//                 bearer.token(), &KEYS.decoding, &Validation::default()
//             )
//             .map_err(|_| AppError::InvalidToken)?;

//         tracing::debug!("claims: {:?}", &data.claims);

//         // token expiration check
//         if time::current_secs_since_epoch() > data.claims.exp {
//             tracing::debug!("token is invalid: {}", bearer.token());
//             Err(AppError::InvalidToken)?
//         }
        
//         let security = Security{ 
//             claims: data.claims,
//             tokens: tokens,
//         };
//         Ok( security )
//     }
// }
