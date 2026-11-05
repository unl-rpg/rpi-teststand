use std::thread;
use std::io::Write;
use std::time::Duration;

/*
 * Reads data from multiple data sensors simultaneously to be written into a file
 */
fn main() {
    create_file();
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    let load_cell = thread::spawn(|| {
            println!("PLACEHOLDER");
            thread::sleep(Duration::from_millis(100));
    });

    let thermocouple = thread::spawn(|| {
            println!("PLACEHOLDER");
            thread::sleep(Duration::from_millis(100));
    });

    let barometer = thread::spawn(|| {
            println!("PLACEHOLDER");
            thread::sleep(Duration::from_millis(100));
    });

    load_cell.join().unwrap();
    thermocouple.join().unwrap();
    barometer.join().unwrap();
}

/*
 * Creates a file to be written in
 */
fn create_file() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    println!("file created!");
}
