// Copyright 2023 ReductStore
// Licensed under the Business Source License 1.1

use crate::api::bucket::BucketSettingsAxum;
use crate::api::middleware::check_permissions;
use crate::api::{Components, HttpError};
use crate::auth::policy::FullAccessPolicy;

use axum::extract::{Path, State};
use axum::headers::HeaderMap;
use std::sync::Arc;

// POST /b/:bucket_name
pub async fn create_bucket(
    State(components): State<Arc<Components>>,
    Path(bucket_name): Path<String>,
    headers: HeaderMap,
    settings: BucketSettingsAxum,
) -> Result<(), HttpError> {
    check_permissions(&components, headers, FullAccessPolicy {}).await?;
    components
        .storage
        .write()
        .await
        .create_bucket(&bucket_name, settings.into())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::tests::{components, headers};
    use crate::api::Components;

    use rstest::rstest;

    use reduct_base::error::ErrorCode;
    use std::sync::Arc;

    #[rstest]
    #[tokio::test]
    async fn test_create_bucket(components: Arc<Components>, headers: HeaderMap) {
        create_bucket(
            State(components),
            Path("bucket-3".to_string()),
            headers,
            BucketSettingsAxum::default(),
        )
        .await
        .unwrap();
    }

    #[rstest]
    #[tokio::test]
    async fn test_create_bucket_already_exists(components: Arc<Components>, headers: HeaderMap) {
        let err = create_bucket(
            State(components),
            Path("bucket-1".to_string()),
            headers,
            BucketSettingsAxum::default(),
        )
        .await
        .err()
        .unwrap();
        assert_eq!(
            err,
            HttpError::new(ErrorCode::Conflict, "Bucket 'bucket-1' already exists",)
        )
    }
}
