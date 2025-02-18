// Copyright 2023 ReductStore
// Licensed under the Business Source License 1.1

use crate::api::middleware::check_permissions;
use crate::api::token::TokenAxum;
use crate::api::{Components, HttpError};
use crate::auth::policy::AuthenticatedPolicy;

use axum::extract::State;
use axum::headers::HeaderMap;
use std::sync::Arc;

// // GET /me
pub async fn me(
    State(components): State<Arc<Components>>,
    headers: HeaderMap,
) -> Result<TokenAxum, HttpError> {
    check_permissions(&components, headers.clone(), AuthenticatedPolicy {}).await?;
    let header = match headers.get("Authorization") {
        Some(header) => header.to_str().ok(),
        None => None,
    };
    Ok(TokenAxum::from(
        components.token_repo.read().await.validate_token(header)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::tests::{components, headers};
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn test_me(components: Arc<Components>, headers: HeaderMap) {
        let token = me(State(components), headers).await.unwrap().0;
        assert_eq!(token.name, "init-token");
    }
}
