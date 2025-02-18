// Copyright 2023 ReductStore
// Licensed under the Business Source License 1.1

use crate::api::middleware::check_permissions;
use crate::api::server::BucketInfoListAxum;
use crate::api::{Components, HttpError};
use crate::auth::policy::AuthenticatedPolicy;
use axum::extract::State;
use axum::headers::HeaderMap;
use std::sync::Arc;

// GET /list
pub async fn list(
    State(components): State<Arc<Components>>,
    headers: HeaderMap,
) -> Result<BucketInfoListAxum, HttpError> {
    check_permissions(&components, headers, AuthenticatedPolicy {}).await?;

    let list = components.storage.read().await.get_bucket_list()?;
    Ok(BucketInfoListAxum::from(list))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::tests::{components, headers};
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn test_list(components: Arc<Components>, headers: HeaderMap) {
        let list = list(State(components), headers).await.unwrap();
        assert_eq!(list.0.buckets.len(), 2);
    }
}
