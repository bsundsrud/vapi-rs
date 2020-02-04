use std::time::Duration;
use vapi_rs::prelude::*;

fn callback(tx: LogTransaction) -> CallbackResult {
    println!(
        "APP-- tx: {} {} {} {:?} {:?}",
        tx.level, tx.vxid, tx.parent_vxid, tx.ty, tx.reason
    );

    return CallbackResult::Continue;
}

fn main() -> Result<(), VarnishError> {
    let mut varnish = Varnish::new();
    varnish.timeout(Duration::from_secs(5));

    let varnish = varnish.build()?;
    let opts = CursorOpts::new().batch();
    varnish.log(None, opts, LogGrouping::Vxid, Box::new(callback))?;
    Ok(())
}
