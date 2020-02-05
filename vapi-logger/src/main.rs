use crossbeam::thread;
use std::sync::mpsc::channel;
use std::time::Duration;
use vapi::prelude::*;

fn _callback(tx: LogTransaction) -> CallbackResult {
    println!(
        "APP-- tx: {} {} {} {:?} {:?}",
        tx.level, tx.vxid, tx.parent_vxid, tx.ty, tx.reason
    );

    CallbackResult::Continue
}

fn main() -> Result<(), VarnishError> {
    thread::scope(|s| {
        let (tx, rx) = channel::<()>();
        let handle = s.spawn(|_| {
            let mut varnish = Varnish::builder();
            varnish.timeout(Duration::from_secs(5));

            let varnish = varnish.build()?;
            let opts = CursorOpts::new().batch();
            varnish.log_builder().opts(opts).start(
                Box::new(move |t| {
                    println!(
                        "APP-- tx: {} {} {} {:?} {:?}",
                        t.level, t.vxid, t.parent_vxid, t.ty, t.reason
                    );
                    CallbackResult::Continue
                }),
                Some(rx),
            )
        });
        std::thread::sleep(Duration::from_secs(10));
        let _ = tx.send(());
        let _ = handle.join();
    })
    .unwrap();
    Ok(())
}
