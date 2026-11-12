use std::thread;
use std::io::Read;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serialport::SerialPort;
use std::time::Duration;

/*
 * Reads data from multiple data sensors simultaneously to be written into a JSON file
 */
fn main() {
    let file = std::fs::File::create("data.jsonl").expect("create failed");
    println!("file created!");
    let mut file1 = file.try_clone().unwrap();
    let mut file2 = file.try_clone().unwrap();
    let mut file3 = file.try_clone().unwrap();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let load_cell = thread::spawn(move || {
        let mut load_cell_port = serialport::new("/dev/ttyUSB0", 115_200) //Note that ports are placeholders
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
        loop {
            let text = read_port(&mut load_cell_port);
            let json_output = serde_json::to_string_pretty(&text);
            file1.write_all(json_output.unwrap().as_bytes()).unwrap();
            file1.write_all(b"\n").unwrap();

            thread::sleep(Duration::from_millis(100));
        }

        
    });

    let thermocouple = thread::spawn(move || {
        let mut thermocouple_port = serialport::new("/dev/ttyUSB0", 115_200) //Note that ports are placeholders
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
        loop {
            let text = read_port(&mut thermocouple_port);
            let json_output = serde_json::to_string_pretty(&text);
            file2.write_all(json_output.unwrap().as_bytes()).unwrap();
            file2.write_all(b"\n").unwrap();

            thread::sleep(Duration::from_millis(100));
        }
    });

    let barometer = thread::spawn(move || {
        let mut barometer_port = serialport::new("/dev/ttyUSB0", 115_200) //Note that ports are placeholders
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

        loop {
            let text = read_port(&mut barometer_port);
            let json_output = serde_json::to_string_pretty(&text);
            file3.write_all(json_output.unwrap().as_bytes()).unwrap();
            file3.write_all(b"\n").unwrap();

            thread::sleep(Duration::from_millis(100));
        }
    });

    load_cell.join().unwrap();
    thermocouple.join().unwrap();
    barometer.join().unwrap();
}

/*
 * Returns a string of our data
 * Reads a port for data and writes it into our JSON File
 */
fn read_port(port: &mut Box<dyn SerialPort>) -> String {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    let bytes_read = port.read(serial_buf.as_mut_slice()).unwrap_or(0);
    let text = String::from_utf8_lossy(&serial_buf[..bytes_read]);
    text.into_owned()
}