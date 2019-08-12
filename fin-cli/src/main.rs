extern crate clap;

mod command;

use command::{get_command, Command};

use fin_lib::{get_revision, get_eeprom, set_eeprom, get_uid};

fn main() {
    match get_command() {
        Command::Revision => println!("{}", get_revision()),
        Command::Eeprom(data) => {
            if let Some(ref eeprom) = data {
                if let None = set_eeprom(eeprom) {
                    println!("Incorrect EEPROM value");
                }
            } else {
                if let Some(eeprom) = get_eeprom() {
                    println!("{}", eeprom);
                }
            }
        },
        Command::Uid => {
            if let Some(uid) = get_uid() {
                println!("{}", uid);
            }
        },
    }
}
