pub mod error;
pub mod vapi;
pub mod vsl;
mod vsm;

pub use crate::vapi::Varnish;
pub use crate::vsl::{
    CallbackResult, CursorOpts, LogCallback, LogGrouping, LogLine, LogTransaction, Reason,
    RecordType, TxType,
};

pub mod prelude {
    pub use crate::error::VarnishError;
    pub use crate::vapi::{Builder, Varnish};
    pub use crate::vsl::{
        CallbackResult, CursorOpts, LogCallback, LogGrouping, LogLine, LogTransaction, Reason,
        RecordType, TxType,
    };
}
