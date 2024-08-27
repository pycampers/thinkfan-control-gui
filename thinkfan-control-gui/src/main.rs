use std::fs::File;
use std::io::{self, stdin, Read, Write}; // Write trait is needed for write_all

fn main() -> io::Result<()> {

    println!("input the speed you want from 0 to 7");

    let mut speed = String::new();
    match io::stdin().read_line(&mut speed) {
        Ok(_) => println!("Correct input"),
        Err(e) => println!("Incorrect input {}", e),
    }

    match set_speed(&speed) {
        Ok(_) => println!("Speed set successfully"),
        Err(e) => eprintln!("Failed to set speed: {}", e),
    }
    Ok(())
}

fn set_speed(speed: &str) -> io::Result<()> {
    let new_speed = format!("level {}", speed); //deve escrever sem o : para funcionar

    let mut file = File::create("/proc/acpi/ibm/fan")?;
    file.write_all(new_speed.as_bytes())?;

    Ok(())
}
