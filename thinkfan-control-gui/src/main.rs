use std::cell::Cell;
use std::fs::File;
use std::io::{self, Read, Write};
use std::rc::Rc;

use gtk::{glib, prelude::*};

use glib::clone;


use gtk::{self, Button,  Orientation};

use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.clock")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let button_increase = Button::builder()
        .label("INCREASE")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("DECREASE")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    match get_current_fan_level() {
        Ok(speed) => {
            number.set(speed);
        }
        Err(e) => {
            eprintln!("Failed to get fan speed Level: {}", e);
        }
    }

    button_increase.connect_clicked(clone!(
        #[weak]
        number,
        #[strong]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            let _ = set_speed(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[strong]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            let _ = set_speed(&number.get().to_string());
        }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    window.set_title(Some("Clock Example"));
    window.set_default_size(260, 400);

    //let time = current_time();
    let label = gtk::Label::default();
    let label_level = gtk::Label::default();

    match get_current_fan_level() {
        Ok(speed) => {
            label_level.set_text(&format!("Fan Level: {}", speed));
        }
        Err(e) => {
            eprintln!("Failed to get fan speed Level: {}", e);
            label_level.set_text("Fan Speed Level: Error");
        }
    }

    match get_current_fan_speed() {
        Ok(speed) => {
            label.set_text(&format!("Fan Speed: {}", speed));
        }
        Err(e) => {
            eprintln!("Failed to get fan speed: {}", e);
            label.set_text("Fan Speed: Error");
        }
    }
    //label.set_text(&fan_speed.to_string());

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&label);
    gtk_box.append(&label_level);

    //window.set_child(Some(&label));
    window.set_child(Some(&gtk_box));

    window.present();

    // we are using a closure to capture the label (else we could also use a normal
    // function)
    let tick = move || {
        match get_current_fan_speed() {
            Ok(speed) => {
                label.set_text(&format!("Fan Speed: {}", speed));
            }
            Err(e) => {
                eprintln!("Failed to get fan speed: {}", e);
                label.set_text("Fan Speed: Error");
            }
        }

        match get_current_fan_level() {
            Ok(speed) => {
                label_level.set_text(&format!("Fan Level: {}", speed));
            }
            Err(e) => {
                eprintln!("Failed to get fan speed Level: {}", e);
                label_level.set_text("Fan Speed Level: Error");
            }
        }

        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
}

fn set_speed(speed: &str) -> io::Result<()> {
    let new_speed = format!("level {}", speed); //deve escrever sem o : para funcionar

    let mut file = File::create("/proc/acpi/ibm/fan")?;
    file.write_all(new_speed.as_bytes())?;

    Ok(())
}

fn get_current_fan_speed() -> io::Result<i32> {
    let mut file = File::open("/proc/acpi/ibm/fan")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents
        .lines()
        .find(|line| line.starts_with("speed:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|speed| speed.trim().parse().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse fan speed"))
}

fn get_current_fan_level() -> io::Result<i32> {
    let mut file = File::open("/proc/acpi/ibm/fan")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents
        .lines()
        .find(|line| line.starts_with("level:"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|speed| speed.trim().parse().ok())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to parse fan level speed",
            )
        })
}
