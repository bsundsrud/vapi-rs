use anyhow::Result;
use config::Config;
use crossbeam::thread;
use crossbeam_channel::{select, unbounded};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use structopt::StructOpt;
use tracing::{error, info};
use transform::LogTransform;
use vapi::prelude::*;

mod parsers;
use std::{path::PathBuf, time::Duration};
use tracing_subscriber::EnvFilter;
mod config;
mod models;
mod output;
mod transform;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(parse(from_os_str), help = "Path to logger config file")]
    config: PathBuf,
}

fn load_config(path: &Path) -> Result<Config> {
    let config_file = File::open(&path)?;
    let mut reader = BufReader::new(config_file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(toml::from_str(&contents)?)
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("vapi_logger=INFO".parse()?))
        .try_init()
        .unwrap();
    let opt = Opt::from_args();
    let config = load_config(&opt.config)?;
    thread::scope(|s| {
        let (_tx, rx) = unbounded::<()>();
        let (tx_reacquired, rx_reacquired) = unbounded::<()>();
        let overrun_watcher_stop_signal = rx.clone();
        let overrun_watcher = s.spawn(move |_| loop {
            select! {
                recv(overrun_watcher_stop_signal) -> _res => {
                    info!("Watchdog received shutdown.");
                    break;
                }
                recv(rx_reacquired) -> res => match res {
                    Ok(_) => error!("Log overrun!"),
                    Err(e) => error!("Watchdog channel error: {}", e)
                }
            }
        });
        let (log_tx, log_rx) = unbounded::<LogTransaction>();
        let log_transform = LogTransform::new().from_config(&config.logging);

        let log_query = config.logging.query.clone();
        let log_consumer = s.spawn(move |_| {
            transform::consume_logs_forever(log_transform, &config.output, log_rx);
        });

        let handle = s.spawn(move |_| {
            let mut varnish = Varnish::builder();
            varnish.timeout(Duration::from_secs(5));

            let varnish = varnish.build()?;
            let opts = CursorOpts::new().batch();
            varnish
                .log_builder()
                .query(&log_query)
                .opts(opts)
                .grouping(LogGrouping::Request)
                .reacquire_and_signal_after_overrun(tx_reacquired)
                .start(
                    Box::new(move |log| {
                        if let Err(e) = log_tx.send(log) {
                            error!("Couldn't send log line: {}", e);
                        }
                        CallbackResult::Continue
                    }),
                    Some(rx),
                )
        });
        let tcp_handle = s.spawn(|_| {});
        let _ = handle.join();
        let _ = overrun_watcher.join();
        let _ = log_consumer.join();
        let _ = tcp_handle.join();
    })
    .unwrap();
    Ok(())
}
