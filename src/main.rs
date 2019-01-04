use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use clap::{Arg, App};

pub mod gpio;
pub mod moist_sensor;
pub mod socket_publisher;

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
        .arg(Arg::with_name("socket")
             .short("s")
             .long("socket")
             .value_name("PATH")
             .help("The path to the created unix socket")
             .required(false)
             .takes_value(true)
             .default_value("./moisture.sock")
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
        .get_matches();

    let gpio_path = cmd.value_of("gpio").unwrap();
    let val_pin = u8::from_str_radix(cmd.value_of("val").unwrap(), 10).unwrap();
    let pwr_pin = u8::from_str_radix(cmd.value_of("pwr").unwrap(), 10).unwrap();
    let socket_path = cmd.value_of("socket").unwrap();
    let sensor_interval = u64::from_str_radix(cmd.value_of("interval").unwrap(), 10).unwrap();

    let teardown = Arc::new(AtomicBool::new(false));

    let signal_teardown = teardown.clone();
	ctrlc::set_handler(move || {
        signal_teardown.store(true, Ordering::SeqCst);
    }).expect("Error setting SIGINT handler");

    let mut gp = gpio::Gpio::new(&gpio_path).unwrap();
    let moist = moist_sensor::MoistSensor::new(pwr_pin, val_pin);
    moist.init(&mut gp).unwrap();

    socket_publisher::run(teardown.clone(), &socket_path, sensor_interval, &moist, &mut gp).unwrap();

    moist.clear(&mut gp).unwrap();
}
