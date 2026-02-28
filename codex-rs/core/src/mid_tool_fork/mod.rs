//! Mid-tool fork module — provides the ability to fork an in-progress tool
//! call into parallel branches for speculative execution.

mod error;

pub use error::ForkError;
pub use error::RetryableError;
