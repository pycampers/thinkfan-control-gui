use std::fs::File;
use std::io::{self, Write}; // Write trait is needed for write_all

fn main() -> io::Result<()> {
    // Call set_speed and handle potential errors
    match set_speed("5") {
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
