use std::fs::File;
use std::io::{self, stdin, Read, Write};
use std::cell::Cell;
use std::rc::Rc;




use glib::{ prelude::*};
use glib::{clone, MainContext};
use glib::source::Priority;

use std::cell::RefCell;



use gtk::prelude::*;
use gtk::{self, glib, Application,Label, ApplicationWindow, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement4";


fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
    //
    //    let mut speed = String::new();
    //    match io::stdin().read_line(&mut speed) {
    //        Ok(_) => println!("Correct input"),
    //        Err(e) => println!("Incorrect input {}", e),
    //    }
    //
    //    match set_speed(&speed) {
    //        Ok(_) => println!("Speed set successfully"),
    //        Err(e) => eprintln!("Failed to set speed: {}", e),
    //    }
    //    Ok(())
}

fn set_speed(speed: &str) -> io::Result<()> {
    let new_speed = format!("level {}", speed); //deve escrever sem o : para funcionar

    let mut file = File::create("/proc/acpi/ibm/fan")?;
    file.write_all(new_speed.as_bytes())?;

    Ok(())
}

fn set_profile() -> () {
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

fn display_config() -> () {
    //needs to display the current config in a box
    //need to be able to createa new config inside the box
    //button save to activate
    // button set current level same as the python one
    // look docs about speed limits, levels
}

fn build_ui(app: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let speed_label = Label::builder()
        .label("Fan S[eed: 0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(
        #[weak]
        number,
        #[strong]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label("INCREASE");
            let _ = set_speed(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[strong]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label("DECREASE");
            let _ = set_speed(&number.get().to_string());
        }
    ));

    let speed = Rc::new(RefCell::new(0));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&speed_label);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
