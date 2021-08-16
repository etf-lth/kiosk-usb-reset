use rmesg;

use std::fs::File;
use std::io::prelude::*;

use std::{thread, time};

fn reset_usb() -> std::io::Result<()> {
    println!("kiosk_usb_reset: Resetting...");

    let mut file = File::create("/sys/bus/usb/devices/1-1.2/authorized")?;

    file.write_all(b"0")?;

    thread::sleep(time::Duration::from_millis(1000));

    file.write_all(b"1")?;

    Ok(())
}

fn check_messages() {
    let entries = rmesg::log_entries(rmesg::Backend::Default, false).unwrap();
    let mut last: time::Duration = time::Duration::from_secs(0);
    for entry in entries {
        last = match entry.timestamp_from_system_start {
            Some(time) => time,
            None => time::Duration::from_secs(0),
        };
    }

    let entries = rmesg::logs_iter(rmesg::Backend::Default, false, false).unwrap();
    for maybe_entry in entries {
        let entry = match maybe_entry {
            Ok(entry) => entry,
            Err(e) => panic!("{}", e),
        };
        let time = match entry.timestamp_from_system_start {
            Some(time) => time,
            None => continue,
        };

        if time < last {
            continue;
        }

        println!("{}", entry);

        if entry.message.contains("usb 1-1.2: reset full-speed USB device number") {
            println!("kiosk_usb_reset: Reset line matched!");

            match reset_usb() {
                Ok(()) => println!("kiosk_usb_reset: Device reset."),
                Err(e) => panic!("{}", e),
            }
        }
    }
}

fn main() {
    println!("kiosk_usb_reset: Started");

    check_messages();
}
