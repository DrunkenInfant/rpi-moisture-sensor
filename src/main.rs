#[macro_use] extern crate serde_json;
use std::sync::{Arc, Mutex};
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
pub mod sensor_setup;
pub mod rabbitmq_publisher;

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

    let gp = Arc::new(Mutex::new(gpio::Gpio::new(&gpio_path).unwrap()));
    let sample_streams = match sensor_setup::setup(&config, gp.clone()) {
        Ok(sf) => sf,
        Err(err) => panic!("Config error {:?}", err)
    };

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
                    sample_streams
                )
            ).expect("runtime exited with error");
        },
        (&_, _) => println!("{}", cmd.usage())
    };

    // TODO Bring back teardown
    //{ sensor.clear(&mut gp.lock().unwrap()).unwrap() };
}
