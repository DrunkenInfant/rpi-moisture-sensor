use std::io::{Write, ErrorKind};
use std::os::unix::net::UnixListener;
use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use std::time::Duration;
use clap::{Arg, App};

pub mod gpio;
pub mod moist_sensor;

fn u32_to_bytes(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn main() {
    let cmd = App::new("Moist sensor server")
        .arg(Arg::with_name("val")
             .long("val")
             .value_name("BCMPIN")
             .help("The bcm pin number to read moisture sensor data from")
             .possible_values(&["17", "27"])
             .required(true)
             .takes_value(true)
         )
        .arg(Arg::with_name("pwr")
             .long("pwr")
             .value_name("BCMPIN")
             .help("The bcm pin number to provide power to moisture sensor")
             .possible_values(&["17", "27"])
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
        .get_matches();

    let val_pin = u8::from_str_radix(cmd.value_of("val").unwrap(), 10).unwrap();
    let pwr_pin = u8::from_str_radix(cmd.value_of("pwr").unwrap(), 10).unwrap();
    let socket_path = cmd.value_of("socket").unwrap();
    let sensor_interval = u64::from_str_radix(cmd.value_of("interval").unwrap(), 10).unwrap();

    let teardown = Arc::new(AtomicBool::new(false));

    let signal_teardown = teardown.clone();
	ctrlc::set_handler(move || {
        signal_teardown.store(true, Ordering::SeqCst);
    }).expect("Error setting SIGINT handler");

    let mut gp = gpio::Gpio::new().unwrap();
    let mut moist = moist_sensor::MoistSensor::new(pwr_pin, val_pin, &mut gp);
    moist.init().unwrap();

    let path = std::path::Path::new(socket_path);
    // Success is not important here
    let _ = std::fs::remove_file(path);
    let listener = UnixListener::bind(socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    while !teardown.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                loop {
                    match stream.write_all(&u32_to_bytes(moist.read().unwrap())) {
                        Ok(()) => std::thread::sleep(Duration::from_secs(sensor_interval)),
                        Err(err) => {
                            match err.kind() {
                                ErrorKind::BrokenPipe => break,
                                _ => {
                                    println!("Socket write error: {}", err);
                                    break;
                                }
                            }
                        }
                    }
                }
            },
            Err(err) => {
                match err.kind() {
                    ErrorKind::WouldBlock => {},
                    _ => {
                        println!("Socket accept error: {}", err);
                        break;
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }

    std::fs::remove_file(path).unwrap();
}
