use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DataErrKind {
    #[error("format error : {0}\n{1:?} ")]
    FormatError(String, Option<String>),
    #[error("not complete")]
    NotComplete,
    #[error("no parse data: {0}")]
    UnParse(String),

    #[error("less data")]
    LessData,
    #[error("empty data")]
    EmptyData,
    #[error("struct less : {0}")]
    LessStc(String),
    #[error("define less : {0}")]
    LessDef(String),
}

impl From<String> for DataErrKind {
    fn from(value: String) -> Self {
        DataErrKind::FormatError(value, None)
    }
}

use derive_more::From;
use orion_error::conversion::ToStructError;
use orion_error::{OrionError, StructError, UvsFrom, UvsReason};

#[derive(Debug, Clone, PartialEq, Serialize, From, OrionError)]
pub enum WparseReason {
    #[orion_error(identity = "biz.plugin")]
    #[from(skip)]
    Plugin(String),
    #[orion_error(identity = "biz.not_match", message = "not match")]
    NotMatch,
    #[orion_error(identity = "biz.line_proc")]
    LineProc(String),
    #[orion_error(transparent)]
    Uvs(UvsReason),
}

pub type WparseError = StructError<WparseReason>;

impl From<DataErrKind> for WparseError {
    fn from(value: DataErrKind) -> Self {
        WparseReason::from_data()
            .to_err()
            .with_detail(format!("{}", value))
    }
}
pub type WparseResult<T> = Result<T, WparseError>;

/// 兼容别名：保留历史命名，方便渐进迁移。
#[deprecated(note = "use `WparseReason` instead")]
pub type WplParseReason = WparseReason;

#[deprecated(note = "use `WparseError` instead")]
pub type WplParseError = WparseError;

#[deprecated(note = "use `WparseResult` instead")]
pub type WplParseResult<T> = WparseResult<T>;
