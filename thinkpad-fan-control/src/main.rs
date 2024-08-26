use std::fs::File;
use std::io::prelude::*;
use std::fs;


fn main() {
    //    let mystirng = "Hello, world!";
    //    let mut file = File::create("hello.txt")?;
    //    file.write_all(mystirng.as_bytes())?;
    //    Ok(())
     let speed = '5';
     let new_speed = format!(
        "status:		enabled\nspeed:		2000\nlevel:          {}",
        speed.to_string()
    );

    fs::write("/proc/acpi/ibm/fan", new_speed).expect("Unable to write file");

//    set_speed('5')
}

fn get_info() {
    //get current tem, and fan speed to display
}

//file format
//status:		enabled
//speed:		3584
//level:		4

fn set_speed(speed: char) -> std::io::Result<()> {
    let new_speed = format!(
        "status:		enabled\nspeed:		2000\nlevel:          {}",
        speed.to_string()
    );
    let mut file = File::create("/proc/acpi/ibm/fan")?;
    println!("Debug: Writing to file:\n{}", new_speed);


    file.write_all(new_speed.as_bytes())?;

    // Debug: Confirm that write was successful
    println!("Debug: Successfully wrote to file");

    Ok(())
}
