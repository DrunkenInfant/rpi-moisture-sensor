#[macro_use] extern crate serde_json;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use clap::{Arg, App, SubCommand};
use futures::{Future, Stream};
use tokio::runtime::Runtime;
use tokio_signal::unix::{Signal, SIGINT, SIGTERM};

pub mod gpio;
pub mod moist_sensor;
pub mod sample_formatter;
pub mod sensor;
pub mod sensor_config;
pub mod sensor_sampler;
pub mod socket_publisher;
pub mod rabbitmq_publisher;

use crate::sensor::Sensor;

fn main() {
    let cmd = App::new("Moist sensor server")
        .arg(Arg::with_name("config")
             .long("config")
             .value_name("PATH")
             .help("Path to config file")
             .required(true)
             .takes_value(true)
         )
        .arg(Arg::with_name("interval")
             .short("i")
             .long("interval")
             .value_name("SECS")
             .help("The interval in seconds between samples")
             .required(false)
             .takes_value(true)
             .default_value("1")
         )
        .arg(Arg::with_name("gpio")
             .long("gpio")
             .value_name("PATH")
             .help("Path GPIO device or simulation file")
             .required(false)
             .takes_value(true)
             .default_value("/dev/gpiomem")
         )
        .subcommand(SubCommand::with_name("socket")
            .about("Publish sensor values on unix socket")
            .arg(Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .value_name("PATH")
                 .help("The path to the created unix socket")
                 .required(false)
                 .takes_value(true)
                 .default_value("./moisture.sock")
             )
        )
        .subcommand(SubCommand::with_name("rabbitmq")
            .about("Publish sensor values on rabbitmq")
            .arg(Arg::with_name("host")
                 .long("host")
                 .short("h")
                 .value_name("HOST")
                 .help("Host and port of rabbitmq server, eg 127.0.0.1:5672")
                 .required(true)
                 .takes_value(true)
             )
            .arg(Arg::with_name("exchange")
                 .long("exchange")
                 .short("e")
                 .value_name("EXCHANGE")
                 .help("Name of the rabbitmq exchange to publish to")
                 .required(true)
                 .takes_value(true)
             )
        )
        .get_matches();

    let config_path = cmd.value_of("config").expect("Config path is required");
    let toml_str = std::fs::read_to_string(config_path)
        .expect(&format!("Error reading file at {}", config_path));
    let config = sensor_config::from_toml(&toml_str)
        .expect(&format!("Error parsing configuration at {}", config_path));
    println!("Using config: {:?}", config);

    let gpio_path = cmd.value_of("gpio").unwrap();
    let sensor_interval = u64::from_str_radix(cmd.value_of("interval").unwrap(), 10).unwrap();

    let gp = Arc::new(Mutex::new(gpio::Gpio::new(&gpio_path).unwrap()));
    let (sensor, formatter) = match config.initialize() {
        Ok(sf) => sf,
        Err(err) => panic!("Config error {:?}", err)
    };
    { sensor.init(&mut gp.lock().unwrap()).unwrap() };

    let sampler = sensor_sampler::SensorSampler::new(sensor, gp.clone(), sensor_interval)
        .map(move |sample| formatter.format(&sample))
        .map_err(failure::Error::from);

    match cmd.subcommand() {
        ("rabbitmq", Some(rmq_cmd)) => {
            let int = Signal::new(SIGINT).flatten_stream().into_future();
            let term = Signal::new(SIGTERM).flatten_stream().into_future();
            let sigf = int.select(term)
                .map(|((v, _), _)| v)
                .map_err(|((err, _), _)| err);
            Runtime::new().unwrap().block_on_all(
                rabbitmq_publisher::run(
                    sigf.shared(),
                    rmq_cmd.value_of("host").unwrap(),
                    rmq_cmd.value_of("exchange").unwrap(),
                    sampler
                )
            ).expect("runtime exited with error");
        },
        ("socket", Some(socket_cmd)) => {
            let teardown = Arc::new(AtomicBool::new(false));
            let signal_teardown = teardown.clone();
            ctrlc::set_handler(move || {
                signal_teardown.store(true, Ordering::SeqCst);
            }).expect("Error setting SIGINT handler");
            socket_publisher::run(
                teardown.clone(),
                socket_cmd.value_of("path").unwrap(),
                sensor_interval,
                sensor.clone(),
                gp.clone()
            ).unwrap();
        }
        (&_, _) => println!("{}", cmd.usage())
    };

    { sensor.clear(&mut gp.lock().unwrap()).unwrap() };
}
