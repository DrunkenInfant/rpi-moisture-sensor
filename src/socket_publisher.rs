use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::unix::net::UnixListener;
use std::io::{Write, ErrorKind};
use std::time::Duration;
use crate::gpio::Gpio;
use crate::moist_sensor::MoistSensor;

#[derive(Debug)]
pub struct Error {
    message: String
}

fn u32_to_bytes(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn run(teardown: Arc<AtomicBool>, socket_path: &str, sensor_interval: u64, moist: MoistSensor, gp: Arc<Mutex<Gpio>>) -> Result<(), Error> {
    let path = std::path::Path::new(socket_path);
    // Success is not important here
    let _ = std::fs::remove_file(path);
    let listener = UnixListener::bind(socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    while !teardown.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                loop {
                    match stream.write_all(&u32_to_bytes(moist.read(&mut gp.lock().unwrap()).unwrap())) {
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

    let _ = std::fs::remove_file(path);

    Ok(())
}
