use crate::error::Result;
use crate::vsl::{CursorOpts, LogCallback, LogGrouping, VarnishLogBuilder};
use crate::vsm::{OpenVSM, VSMBuilder};
use std::time::Duration;

pub struct Varnish {
    shm: OpenVSM,
}

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
    pub fn new() -> Builder {
        Builder::new()
    }

    pub fn log(
        &self,
        query: Option<String>,
        cursor_opts: CursorOpts,
        grouping: LogGrouping,
        callback: LogCallback,
    ) -> Result<()> {
        let mut builder = VarnishLogBuilder::new();
        builder.grouping(grouping);
        builder.cursor_opts(cursor_opts);
        if let Some(query) = query {
            builder.query(query);
        }
        builder.execute(&self.shm, callback)
    }
}
