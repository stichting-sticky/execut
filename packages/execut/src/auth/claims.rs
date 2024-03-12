use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{users::Role, Context, Error};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub subject: Uuid,
    #[serde(rename = "exp")]
    pub expires_at: usize,
    pub role: Role,
}

#[async_trait]
impl FromRequestParts<Context> for Claims {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Context,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidToken)?;

        // Decode the user data
        let token_data =
            decode::<Claims>(bearer.token(), &state.keys.decoding, &Validation::default())
                .map_err(|_| Error::InvalidToken)?;

        let TokenData { claims, .. } = token_data;

        Ok(claims)
    }
}
