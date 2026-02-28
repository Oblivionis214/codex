//! Fork error types and retry classification.

use thiserror::Error;

/// Errors that can occur during mid-tool fork operations.
#[derive(Error, Debug)]
pub enum ForkError {
    /// Fork precondition not met (wrong state, fork count exceeded, pending call mismatch).
    #[error("fork precondition failed: {0}")]
    PreconditionFailed(String),

    /// LogicalSnapshot capture failure.
    #[error("snapshot failed: {0}")]
    SnapshotFailed(String),

    /// Session factory failure — retryable (3x with 100/200/400ms backoff).
    #[error("session creation failed: {0}")]
    SessionCreationFailed(String),

    /// Invalid `_fork` JSON structure.
    #[error("directive parse failed: {0}")]
    DirectiveParseFailed(String),

    /// Oneshot channel send failure (receiver dropped).
    #[error("result delivery failed: {0}")]
    ResultDeliveryFailed(String),

    /// ForkState machine received an invalid transition.
    #[error("invalid state transition")]
    InvalidStateTransition,

    /// Fork operation exceeded the configured timeout.
    #[error("fork operation timed out")]
    Timeout,
}

/// Trait for classifying errors as retryable or non-retryable.
pub trait RetryableError {
    /// Returns `true` if the operation that produced this error can be retried.
    fn is_retryable(&self) -> bool;
}

impl RetryableError for ForkError {
    fn is_retryable(&self) -> bool {
        matches!(self, ForkError::SessionCreationFailed(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precondition_failed_is_not_retryable() {
        let err = ForkError::PreconditionFailed("bad state".into());
        assert!(!err.is_retryable());
    }

    #[test]
    fn snapshot_failed_is_not_retryable() {
        let err = ForkError::SnapshotFailed("capture error".into());
        assert!(!err.is_retryable());
    }

    #[test]
    fn session_creation_failed_is_retryable() {
        let err = ForkError::SessionCreationFailed("connection refused".into());
        assert!(err.is_retryable());
    }

    #[test]
    fn directive_parse_failed_is_not_retryable() {
        let err = ForkError::DirectiveParseFailed("invalid json".into());
        assert!(!err.is_retryable());
    }

    #[test]
    fn result_delivery_failed_is_not_retryable() {
        let err = ForkError::ResultDeliveryFailed("receiver dropped".into());
        assert!(!err.is_retryable());
    }

    #[test]
    fn invalid_state_transition_is_not_retryable() {
        let err = ForkError::InvalidStateTransition;
        assert!(!err.is_retryable());
    }

    #[test]
    fn timeout_is_not_retryable() {
        let err = ForkError::Timeout;
        assert!(!err.is_retryable());
    }

    #[test]
    fn display_precondition_failed() {
        let err = ForkError::PreconditionFailed("bad state".into());
        assert_eq!(err.to_string(), "fork precondition failed: bad state");
    }

    #[test]
    fn display_snapshot_failed() {
        let err = ForkError::SnapshotFailed("capture error".into());
        assert_eq!(err.to_string(), "snapshot failed: capture error");
    }

    #[test]
    fn display_session_creation_failed() {
        let err = ForkError::SessionCreationFailed("connection refused".into());
        assert_eq!(
            err.to_string(),
            "session creation failed: connection refused"
        );
    }

    #[test]
    fn display_directive_parse_failed() {
        let err = ForkError::DirectiveParseFailed("invalid json".into());
        assert_eq!(err.to_string(), "directive parse failed: invalid json");
    }

    #[test]
    fn display_result_delivery_failed() {
        let err = ForkError::ResultDeliveryFailed("receiver dropped".into());
        assert_eq!(err.to_string(), "result delivery failed: receiver dropped");
    }

    #[test]
    fn display_invalid_state_transition() {
        let err = ForkError::InvalidStateTransition;
        assert_eq!(err.to_string(), "invalid state transition");
    }

    #[test]
    fn display_timeout() {
        let err = ForkError::Timeout;
        assert_eq!(err.to_string(), "fork operation timed out");
    }

    #[test]
    fn fork_error_implements_std_error() {
        fn assert_std_error<T: std::error::Error>() {}
        assert_std_error::<ForkError>();
    }
}
