use anyhow::Result;
use config::Config;
use crossbeam::thread;
use crossbeam_channel::bounded;
use crossbeam_channel::{select, unbounded};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use structopt::StructOpt;
use tracing::{error, info};
use vapi::prelude::*;
use vapi::vsl::LogRecord;

use std::{path::PathBuf, time::Duration};
use tracing_subscriber::EnvFilter;

mod config;
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
                    Err(e) => {
                        error!("Watchdog channel error: {}", e);
                        break;
                    },
                }
            }
        });
        let (log_tx, log_rx) = bounded::<LogRecord>(1000);
        let log_transform = config::transform_from_config(&config.logging);

        let log_query = config.logging.query.clone();
        let output_config = config.output;
        let log_consumer = s.spawn(move |_| {
            transform::consume_logs_forever(&output_config, log_rx);
        });
        let input_config = config.input;
        let logging_config = config.logging;
        let handle = s.spawn(move |_| {
            let mut varnish = Varnish::builder();
            varnish.timeout(Duration::from_secs(input_config.connect_timeout_secs));
            if let Some(path) = input_config.path {
                varnish.path(&path);
            }
            let varnish = match varnish.build() {
                Ok(v) => v,
                Err(e) => {
                    error!("Couldn't connect to varnish: {}", e);
                    return Err(e);
                }
            };

            let mut opts = CursorOpts::new().batch();
            if logging_config.tail {
                opts = opts.tail();
            }

            let type_filter: Vec<TxType> = logging_config
                .type_filter
                .iter()
                .map(|&t| t.into())
                .collect();

            let reason_filter: Vec<Reason> = logging_config
                .reason_filter
                .iter()
                .map(|&t| t.into())
                .collect();

            let res = varnish
                .log_builder()
                .query(&log_query)
                .opts(opts)
                .grouping(logging_config.grouping)
                .type_filter(type_filter)
                .reason_filter(reason_filter)
                .reacquire_and_signal_after_overrun(tx_reacquired)
                .start(log_tx, Some(rx), log_transform);
            match res {
                Err(ref e) => error!("Varnish logging failed: {}", e),
                _ => {}
            }
            res
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
