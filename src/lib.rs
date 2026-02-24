//! Minimal Plugin API for WPL parsers.
//! This crate defines core types for plugin development while maintaining
//! compatibility with the wp-lang ecosystem.

use std::sync::Arc;

use wp_model_core::model::DataRecord;

mod error;
pub use error::{WparseError, WparseReason, WparseResult};
#[allow(deprecated)]
pub use error::{WplParseError, WplParseReason, WplParseResult};
use wp_model_core::raw::RawData;
// Re-export necessary types from wp-lang that we still need

/// Result type for plugin parsing operations.
///
/// On success, returns a tuple of `(DataRecord, remaining_raw)`.
/// On failure, returns a WparseError (旧名称 `WplParseError` 仍可用，但已弃用)。
pub type DataResult = Result<(DataRecord, RawData), WparseError>;

/// Trait for pipeline data processing operations.
///
/// This trait defines the interface for components that process RawData
/// within a data pipeline, transforming it from one format to another
/// (e.g., base64 decoding, hex decoding, string unescaping, etc.).
///
/// Pipeline processors are executed in sequence as part of a data processing
/// pipeline, with the output of one processor becoming the input of the next.
pub trait PipeProcessor {
    /// Process the input data and return the transformed result.
    ///
    /// # Arguments
    /// * `data` - The input data to be processed
    ///
    /// # Returns
    /// The processed data in the appropriate output format
    fn process(&self, data: RawData) -> WparseResult<RawData>;

    /// Get the name/identifier of this pipeline processor.
    ///
    /// # Returns
    /// A string slice representing the processor name
    fn name(&self) -> &'static str;
}

pub type PipeHold = Arc<dyn PipeProcessor + Send + Sync>;

#[cfg(test)]
mod tests {
    use super::RawData;
    use bytes::Bytes;
    use std::sync::Arc;

    #[test]
    fn rawdata_as_bytes_and_len_cover_all_variants() {
        let text = RawData::from_string("hello");
        assert_eq!(text.as_bytes(), b"hello");
        assert_eq!(text.len(), 5);
        assert!(!text.is_zero_copy());

        let bytes = RawData::Bytes(Bytes::from_static(b"bin"));
        assert_eq!(bytes.as_bytes(), b"bin");
        assert_eq!(bytes.len(), 3);
        assert!(bytes.to_bytes().eq(&Bytes::from_static(b"bin")));
        assert!(!bytes.is_zero_copy());

        let arc = Arc::new(vec![1u8, 2, 3, 4]);
        let arc_raw = RawData::from_arc_bytes(arc.clone());
        assert_eq!(arc_raw.as_bytes(), arc.as_slice());
        assert_eq!(arc_raw.len(), 4);
        assert!(arc_raw.is_zero_copy());
        let bytes_from_arc = arc_raw.to_bytes();
        assert_eq!(bytes_from_arc.as_ref(), &[1, 2, 3, 4]);

        let owned = RawData::from_arc_bytes(Arc::new(vec![5u8, 6, 7]));
        let converted = owned.into_bytes();
        assert_eq!(converted.as_ref(), &[5, 6, 7]);
    }

    #[test]
    fn rawdata_is_empty_handles_all_variants() {
        assert!(RawData::from_string("").is_empty());
        assert!(RawData::Bytes(Bytes::new()).is_empty());
        assert!(RawData::from_arc_bytes(Arc::new(vec![])).is_empty());
        assert!(!RawData::from_string("x").is_empty());
    }
}
