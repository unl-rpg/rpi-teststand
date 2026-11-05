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
    let data_sensor1 = thread::spawn(|| {
        println!("PLACEHOLDER");
        thread::sleep(Duration::from_millis(100));
    });

    let data_sensor2 = thread::spawn(|| {
        println!("PLACEHOLDER");
        thread::sleep(Duration::from_millis(100));
    });

    let data_sensor3 = thread::spawn(|| {
        println!("PLACEHOLDER");
        thread::sleep(Duration::from_millis(100));
    });

    data_sensor1.join().unwrap();
    data_sensor2.join().unwrap();
    data_sensor3.join().unwrap();
}

/*
 * Creates a file to be written in
 */
fn create_file() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    println!("file created!");
}
