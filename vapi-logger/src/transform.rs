use crate::config::OutputConfig;
use crate::metrics::{RECONNECT_COUNTER, SENT_COUNTER, SENT_HISTO};
use anyhow::{anyhow, Result};
use crossbeam::select;
use crossbeam_channel::Receiver;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::{net::SocketAddr, net::ToSocketAddrs};
use tracing::{error, info};
use vapi::vsl::LogRecord;
fn send_to_stdout(rx: Receiver<LogRecord>) -> Result<()> {
    loop {
        select! {
            recv(rx) -> res => {
                let log = match res {
                    Ok(l) => l,
                    Err(e) => {
                        error!("Error recv: {}", e);
                        continue;
                    },
                };
                SENT_COUNTER.inc();
                let timer = SENT_HISTO.start_timer();
                println!("-----------------------------------------");
                println!("{}", serde_json::to_string_pretty(&log).unwrap());
                timer.observe_duration();
            }
        }
    }
}

fn loop_until_connected(
    addr: &SocketAddr,
    timeout: Duration,
    retry_interval: Duration,
) -> TcpStream {
    loop {
        match TcpStream::connect_timeout(addr, timeout) {
            Ok(s) => {
                info!("Connected to {}", addr);
                return s;
            }
            Err(e) => {
                error!("Failed to connect: {}", e);
                std::thread::sleep(retry_interval);
            }
        }
    }
}

fn send_to_tcp(
    rx: Receiver<LogRecord>,
    host: &str,
    port: u16,
    timeout: u64,
    retry_interval: u64,
    sender_threads: u64,
) -> Result<()> {
    let addr: SocketAddr = (host, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow!("Invalid addr: {}:{}", host, port))?;
    let timeout = Duration::from_secs(timeout);
    let retry_interval = Duration::from_secs(retry_interval);

    let _ = crossbeam::thread::scope(|s| {
        let mut handles = Vec::new();
        for i in 0..sender_threads {
            let h = s.spawn(|_| -> ! {
                let mut stream = loop_until_connected(&addr, timeout, retry_interval);
                loop {
                    select! {
                        recv(rx) -> res => {
                            let log = match res {
                                Ok(l) => l,
                                Err(e) => {
                                    error!("Error in recv: {}", e);
                                    continue;
                                },
                            };
                            SENT_COUNTER.inc();
                            let mut json: String = match serde_json::to_string(&log) {
                                Ok(j) => j,
                                Err(e) => {
                                    error!("Couldn't transform struct: {}", e);
                                    continue;
                                },
                            };
                            json.push('\n');
                            let timer = SENT_HISTO.start_timer();
                            let res =  stream.write(json.as_bytes());
                            timer.observe_duration();
                            if let Err(e) = res {
                                error!("Error writing to TCP socket: {}", e);
                                stream = loop_until_connected(&addr, timeout, retry_interval);
                                RECONNECT_COUNTER.inc();
                            }
                        }
                    }
                }
            });
            info!("Started sender thread {}", i);
            handles.push(h);
        }
        for handle in handles {
            let _ = handle.join();
        }
    });
    Ok(())
}

fn null_consumer(rx: Receiver<LogRecord>) -> Result<()> {
    loop {
        select! {
            recv(rx) -> res => {
                match res {
                    Ok(_) => {
                        SENT_COUNTER.inc();
                    }
                    Err(e) => { error!("Error in transform: {}", e)}
                }
            }
        }
    }
}

pub fn consume_logs_forever(output: &OutputConfig, rx: Receiver<LogRecord>) {
    let res = match output {
        OutputConfig::Stdout => send_to_stdout(rx),
        OutputConfig::Tcp {
            host,
            port,
            connect_timeout_secs,
            retry_interval_secs,
            sender_threads,
        } => send_to_tcp(
            rx,
            host,
            *port,
            *connect_timeout_secs,
            *retry_interval_secs,
            *sender_threads,
        ),
        OutputConfig::Null => null_consumer(rx),
    };
    if let Err(e) = res {
        error!("Output failure: {}", e);
        std::process::exit(1);
    }
}
