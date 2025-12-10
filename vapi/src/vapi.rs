use crate::error::Result;
use crate::vsl::transform::LogTransform;
use crate::vsl::{CursorOpts, LogGrouping, LogRecord, VarnishLogBuilder};
use crate::vsm::{OpenVSM, VSMBuilder};
use crate::{Reason, TxType};
use crossbeam_channel::{Receiver, Sender};
use std::time::Duration;

pub struct Varnish {
    shm: OpenVSM,
}

#[derive(Default, Debug)]
pub struct Builder {
    path: Option<String>,
    timeout: Option<Duration>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            path: None,
            timeout: None,
        }
    }

    pub fn path<P: Into<String>>(&mut self, path: P) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    pub fn timeout(&mut self, t: Duration) -> &mut Self {
        self.timeout = Some(t);
        self
    }

    pub fn build(self) -> Result<Varnish> {
        let mut builder = VSMBuilder::new()?;
        if let Some(p) = self.path {
            builder.path(p);
        }
        if let Some(t) = self.timeout {
            builder.timeout(t);
        }
        Ok(Varnish {
            shm: builder.attach()?,
        })
    }
}

impl Varnish {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn log_builder(&self) -> LoggingBuilder<'_> {
        LoggingBuilder::new(&self.shm)
    }
}
#[derive(Debug)]
pub struct LoggingBuilder<'vsm> {
    vsm: &'vsm OpenVSM,
    query: Option<String>,
    opts: CursorOpts,
    grouping: LogGrouping,
    reacquire: bool,
    reacquire_signal: Option<Sender<()>>,
    type_filter: Vec<TxType>,
    reason_filter: Vec<Reason>,
}

impl<'vsm> LoggingBuilder<'vsm> {
    fn new(vsm: &'vsm OpenVSM) -> LoggingBuilder<'vsm> {
        LoggingBuilder {
            vsm,
            query: None,
            opts: CursorOpts::new(),
            grouping: LogGrouping::Vxid,
            reacquire: false,
            reacquire_signal: None,
            type_filter: Vec::new(),
            reason_filter: Vec::new(),
        }
    }

    pub fn query<S: Into<String>>(mut self, query: S) -> Self {
        let q = query.into();
        if !q.is_empty() {
            self.query = Some(q);
        }
        self
    }

    pub fn opts(mut self, c: CursorOpts) -> Self {
        self.opts = c;
        self
    }

    pub fn grouping(mut self, grouping: LogGrouping) -> Self {
        self.grouping = grouping;
        self
    }

    pub fn type_filter<V: Into<Vec<TxType>>>(mut self, types: V) -> Self {
        self.type_filter = types.into();
        self
    }

    pub fn reason_filter<V: Into<Vec<Reason>>>(mut self, reasons: V) -> Self {
        self.reason_filter = reasons.into();
        self
    }

    pub fn reacquire_after_overrun(mut self) -> Self {
        self.reacquire = true;
        self
    }

    pub fn reacquire_and_signal_after_overrun(mut self, tx: Sender<()>) -> Self {
        self.reacquire = true;
        self.reacquire_signal = Some(tx);
        self
    }

    pub fn start(
        self,
        log_sender: Sender<LogRecord>,
        stop_channel: Option<Receiver<()>>,
        transform: LogTransform,
    ) -> Result<()> {
        let mut builder = VarnishLogBuilder::new(log_sender, transform);
        builder.grouping(self.grouping);
        builder.cursor_opts(self.opts);
        if let Some(tx) = self.reacquire_signal {
            builder.reacquire_and_notify_if_overrun(tx);
        } else if self.reacquire {
            builder.reacquire_if_overrun();
        }
        if let Some(query) = self.query {
            builder.query(query);
        }
        builder.type_filter(self.type_filter);
        builder.reason_filter(self.reason_filter);
        builder.execute(self.vsm, stop_channel)
    }
}
