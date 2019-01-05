use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use clap::{Arg, App, SubCommand};
use futures::{Future, Stream};
use tokio::runtime::Runtime;
use tokio_signal::unix::{Signal, SIGINT, SIGTERM};

pub mod gpio;
pub mod moist_sensor;
pub mod socket_publisher;
pub mod rabbitmq_publisher;

fn main() {
    let cmd = App::new("Moist sensor server")
        .arg(Arg::with_name("val")
             .long("val")
             .value_name("BCMPIN")
             .help("The bcm pin number to read moisture sensor data from")
             .possible_values(&["4", "5", "6", "13", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27"])
             .required(true)
             .takes_value(true)
         )
        .arg(Arg::with_name("pwr")
             .long("pwr")
             .value_name("BCMPIN")
             .help("The bcm pin number to provide power to moisture sensor")
             .possible_values(&["4", "5", "6", "13", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27"])
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

    let gpio_path = cmd.value_of("gpio").unwrap();
    let val_pin = u8::from_str_radix(cmd.value_of("val").unwrap(), 10).unwrap();
    let pwr_pin = u8::from_str_radix(cmd.value_of("pwr").unwrap(), 10).unwrap();
    let sensor_interval = u64::from_str_radix(cmd.value_of("interval").unwrap(), 10).unwrap();

    let gp = Arc::new(Mutex::new(gpio::Gpio::new(&gpio_path).unwrap()));
    let moist = moist_sensor::MoistSensor::new(pwr_pin, val_pin);
    { moist.init(&mut gp.lock().unwrap()).unwrap() };

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
                    sensor_interval,
                    moist.clone(),
                    gp.clone()
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
                socket_cmd.value_of("socket").unwrap(),
                sensor_interval,
                moist.clone(),
                gp.clone()
            ).unwrap();
        }
        (&_, _) => println!("{}", cmd.usage())
    };

    { moist.clear(&mut gp.lock().unwrap()).unwrap() };
}
