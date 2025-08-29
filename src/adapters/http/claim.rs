use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::domain::errors::DomainError;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Claims {
    sub: String,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = DomainError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(bearer): TypedHeader<Authorization<Bearer>> =
            parts.extract().await.map_err(|_| DomainError::Internal)?;
        let token = bearer.token();
        if token.is_empty() {
            return Err(DomainError::Internal);
        }

        Ok(Claims {
            sub: token.to_string(),
        })
    }
}
