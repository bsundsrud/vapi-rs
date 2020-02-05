use std::time::Duration;
use vapi::prelude::*;

fn callback(tx: LogTransaction) -> CallbackResult {
    println!(
        "APP-- tx: {} {} {} {:?} {:?}",
        tx.level, tx.vxid, tx.parent_vxid, tx.ty, tx.reason
    );

    CallbackResult::Continue
}

fn main() -> Result<(), VarnishError> {
    let mut varnish = Varnish::builder();
    varnish.timeout(Duration::from_secs(5));

    let varnish = varnish.build()?;
    let opts = CursorOpts::new().batch();
    let log_builder = varnish.log_builder().opts(opts);
    log_builder.build(Box::new(callback), None)?;
    Ok(())
}
