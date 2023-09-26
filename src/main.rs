use core::panic;
use std::io::{stdin, Read};
use std::process::Command;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Reminder {
    id: i32,
    text: String,
}

impl Reminder {
    fn get_text(self) -> String {
        self.text
    }

    fn get_id(self) -> i32 {
        self.id
    }
}

fn take_user_input() -> Result<String, std::io::Error> {
    let mut buffer: String = String::new();
    stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

// For Windows, in Linux it should be "clear"
fn clear_console() {
    let _ = Command::new("cls").status().unwrap();
}


// Read the csv file 'reminders.csv' at the root directory and add the reminders struct to the reminders vector
fn read_reminders_from_csv(reminders: &mut Vec<Reminder>) -> Result<(), std::io::Error> {
    let mut reader = match Reader::from_path("reminders.csv") {
        Ok(r) => r,
        Err(e) => panic!("Could not find file reminders.csv, please create it in the root directory")
    };

    for result in reader.deserialize() {
        let reminder: Reminder = result?;
        println!("{:?}", reminder);
        reminders.push(reminder);
    }

    Ok(())
}

fn list_commands() {
    clear_console();
    println!("Welcome to the reminder CLI app!");
    println!("Available commands are:");
    println!("'add': add a new reminder");
    println!("'view': view your reminders");
    println!("'delete': delete a selected reminder");
}

fn add_reminder(reminders: &mut Vec<Reminder>) {
    let user_input: String = match take_user_input() {
        Ok(buffer) => buffer,
        Err(e) => panic!("Error: user input is invalid UTF-8") 
    };

    let reminder: Reminder = Reminder {
        id: 0, 
        text: user_input
    };

    reminders.push(reminder);
}

fn view_reminders(reminders: &Vec<Reminder>) {
    for reminder in reminders.iter() {
        println!("{}: {}", reminder.id, reminder.text)
    }
}

fn delete_reminder(reminders: &mut Vec<Reminder>) {}

fn write_to_csv(reminders: &Vec<Reminder>) {}

fn main() {

    let mut reminders: Vec<Reminder> = Vec::new();

    // mutate reminders in place and check the possible errors
    match read_reminders_from_csv(&mut reminders) {
        Ok(_) => println!("Succesfully read reminders from csv file"),
        Err(e) => panic!("Error reading from csv: {}", e)
    };

    loop {
        // Take the user input
        let user_input: String = match take_user_input() {
            Ok(buffer) => buffer,
            Err(e) => panic!("Error: user input is invalid UTF-8") 
        };

        // Match on the user input as a &str        
        match user_input.as_str() {
            "quit" => break,
            "help" => list_commands(),
            "add" => add_reminder(&mut reminders),
            "view" => view_reminders(&reminders),
            "delete" => delete_reminder(&mut reminders),
            _ => continue
        }
        write_to_csv(&reminders);
    }

    
    clear_console();
    println!("Thank you for using this app");
}
