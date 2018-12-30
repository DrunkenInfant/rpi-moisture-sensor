use std::io::{Write, ErrorKind};
use std::os::unix::net::UnixListener;
use std::time::Duration;

pub mod gpio;
pub mod moist_sensor;

const VAL_PIN: u8 = 17;
const PWR_PIN: u8 = 27;
const MOIST_INTERVALL: u64 = 1;
const SOCKET_PATH: &str = "./moist1.sock";

fn u32_to_bytes(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn main() {
    let mut gp = gpio::Gpio::new().unwrap();
    let mut moist = moist_sensor::MoistSensor::new(PWR_PIN, VAL_PIN, &mut gp);
    moist.init().unwrap();

    let path = std::path::Path::new(SOCKET_PATH);
    // Success is not important here
    let _ = std::fs::remove_file(path);
    let listener = UnixListener::bind(SOCKET_PATH).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                loop {
                    match stream.write_all(&u32_to_bytes(moist.read().unwrap())) {
                        Ok(()) => std::thread::sleep(Duration::from_secs(MOIST_INTERVALL)),
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
                println!("Socket accept error: {}", err);
                break;
            }
        }
    }

    std::fs::remove_file(path).unwrap();
}
