use thingz::config::Settings;
use thingz::commands::{Commands, Cli};

#[macro_use]
extern crate log;

use chrono::prelude::*;
use clap::Parser;
use file_rotate::suffix::{DateFrom, FileLimit};
use rumqttc::{AsyncClient, ConnectionError, Incoming, MqttOptions, QoS};
// use config::Config;
use file_rotate::{
    compression::Compression, suffix::AppendTimestamp, ContentLimit, FileRotate, TimeFrequency,
};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use std::io::Write;
use std::process::exit;
use std::str;
use std::time::Duration;


async fn mqtt_stream_topic(cfg: &Settings) {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", &cfg.mqtt.host, cfg.mqtt.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    info!(
        "[{}] Starting listeners on [{}:{}]",
        cfg.mqtt.topic, cfg.mqtt.host, cfg.mqtt.port
    );
    let mut log = FileRotate::new(
        &cfg.logs.path,
        // AppendCount::new(10),
        AppendTimestamp::with_format(
            "%Y%m%dT%H",
            FileLimit::MaxFiles(cfg.logs.files),
            DateFrom::Now,
        ),
        ContentLimit::Time(TimeFrequency::Hourly),
        Compression::OnRotate(1),
        None,
    );

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(&cfg.mqtt.topic, QoS::AtMostOnce)
        .await
        .unwrap();
    // println!("Path {}", (&log.log_paths()[0]));

    loop {
        let res = eventloop.poll().await;
        match res {
            Ok(notification) => {
                match notification {
                    rumqttc::Event::Incoming(Incoming::Publish(msg)) => {
                        let s = match str::from_utf8(&msg.payload) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                        // let msg: Packet = notification as Packet;
                        // info!("[{}] {}", msg.topic, s);
                        writeln!(log, "{} {} {}", Utc::now(), msg.topic, s).unwrap();
                    }
                    _ => {
                        // println!("NOTIFICATION {:?}", notification);
                    }
                };
            }
            Err(ConnectionError::Io(error)) => {
                if error.kind() == std::io::ErrorKind::ConnectionAborted
                    || error.kind() == std::io::ErrorKind::ConnectionRefused
                    || error.kind() == std::io::ErrorKind::ConnectionReset
                {
                    println!("Failed to connect to the server. Error: {error:?}");
                    return;
                }
                println!("Connection error: {error:?}");
                exit(1);
            }
            _ => {}
        }
    }
}

async fn s3_archive(cfg: &Settings) -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;
    sched.add(
        Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        })?
    ).await?;
    sched.start().await?;

    // Wait while the jobs run
    tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}

#[tokio::main]
async fn main() {
    // RUST_LOG=debug
    env_logger::init();
    let cli = Cli::parse();
    let settings = Settings::new(cli.config).unwrap();
    info!("{:?}", settings);

    match &cli.command {
        Commands::Mqtt => {
            mqtt_stream_topic(&settings).await;
        },
        Commands::S3 => {
            let _ = s3_archive(&settings).await;
        }
    }
}
