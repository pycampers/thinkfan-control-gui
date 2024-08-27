use std::fs::File;
use std::io::{self, stdin, Read, Write};
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


fn set_profile() -> (){
    //read from txt, json, etc
    // text will be form:
    // TEMP TEMP level
    // TEMP TEMP level
    // TEMP TEMP level
    // TEMP TEMP level
    // TEMP TEMP level
    // TEMP TEMP level
    // TEMP TEMP level
    // 7 levels for this laptop .....
    // needs to be something like a daemon running

}


fn display_config() -> (){
    //needs to display the current config in a box
    //need to be able to createa new config inside the box
    //button save to activate
    // button set current level same as the python one
    // look docs about speed limits, levels
}

