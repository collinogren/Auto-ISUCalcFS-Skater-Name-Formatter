/*
    Copyright (c) 2024 collinogren

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    // Get access to the Windows clipboard.
    let mut ctx = ClipboardContext::new().unwrap();
    // Store the last content to detect changes in clipboard state.
    let mut last_content: Option<String> = None;
    // Atomic ref counted pointer with interior mutability so that another thread can control
    // whether or not the program is running.
    let should_run: Arc<RwLock<Run>> = Arc::new(RwLock::new(Run::new(true)));

    // Create a new thread that looks for the user to enter 'q' in the terminal.
    check_for_exit(should_run.clone());

    while should_run.read().unwrap().should_run {
        // Sleep for 50 milliseconds to prevent there from being a lock on the clipboard.
        sleep(Duration::from_millis(50));

        // Get the clipboard contents if it exists as a string.
        let content = match ctx.get_contents() {
            Ok(contents) => { contents }
            Err(_) => { continue }
        };

        // Check if content is none or does not equal the last content.
        // Short-circuit evaluation prevents an error on last_content = None.
        if last_content.is_none() || !content.eq(&last_content.clone().unwrap()) {
            // Set the clipboard contents to the formatted contents if the formatted contents
            // could be created or do not set the contents if they could not be created.
            // Either way, set last content to the current clipboard content.
            ctx.set_contents(match format_full_name(content.clone()) {
                None => {
                    //println!("Failed to format contents");
                    last_content = Some(content);
                    continue
                }
                Some(contents) => {
                    //println!("Successfully formatted contents to \"{}\"", contents);
                    last_content = Some(contents.clone());
                    contents
                }
            }).unwrap_or(()); // Should never error probably.
        } else {
            continue
        }
    }
}

fn format_full_name(name: String) -> Option<String> {
    // split the name by "," and retrieve the first segment of the split string. This gives only the
    // skaters first and last names with no other data.
    let name = name.split(",").nth(0);

    // If the split operation gave none as a result then stop right there.
    if name.is_none() {
        //println!("Input was none");
        return None;
    }

    //println!("{}", name.unwrap());

    // Split by a space which separates the first and last names.
    let mut name_split = name.unwrap().split(" ");

    // Get the first name by getting the first part of the split.
    let first_name = name_split.nth(0);

    // Get the last name by getting the last part of the split.
    let last_name = name_split.last();

    // If either parts are none, stop right there.
    if first_name.is_none() || last_name.is_none() {
        //println!("Either first or last name is none");
        return None;
    }

    //println!("{}", first_name.unwrap());
    //println!("{}", last_name.unwrap());

    // Format the first name into the first four letters of the first name.
    let first_name_first_four = format_individual_name(first_name.unwrap().to_string());

    // Format the last name into the last four letters of the last name.
    let last_name_first_four = format_individual_name(last_name.unwrap().to_string());

    //println!("{}{}", last_name_first_four, first_name_first_four);

    //println!("Success");

    // Return the two first four strings in last name, first name order (lastfirs).
    return Some(format!("{}{}", last_name_first_four, first_name_first_four));
}

// Retrieve the first four letters of a single name.
fn format_individual_name(name: String) -> String {
    let mut name_first_four = String::new();

    // Create an iterator to easily retrieve each character from the string.
    let mut name_iter = name.chars();

    // Loop four times
    for _ in 0..4 {
        // In every iteration of the loop, add the next character from the input string to the
        // mutable name_first_four string.
        name_first_four = format!("{}{}", name_first_four, match name_iter.next() {
            None => { break } // If the string ends before four then break out of the loop
            Some(c) => { c }
        });
    }
    
    return name_first_four;
}

fn check_for_exit(should_run: Arc<RwLock<Run>>) {
    // Create a new thread.
    thread::spawn(move || {
        let mut check_input = true;
        println!("Auto ISUCalc Skater Name Formatter is running.\nType 'q' and press enter to exit.");
        while check_input {
            // Read user input
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            // Check if user input matches the prompt to quit.
            if line.to_lowercase().trim().eq("q") {
                check_input = false;

                // Write to the Arc<RwLock> that the program should not run anymore, thus stopping
                // the program on the main thread.
                should_run.write().unwrap().set_should_run(false);
                println!("Program terminated.")
            }
        }
    });
}

// Struct for interior mutability with the Arc<RwLock> should_run.
struct Run {
    should_run: bool,
}

impl Run {
    fn new(should_run: bool) -> Self {
        Self {
            should_run
        }
    }
    fn set_should_run(&mut self, should_run: bool) {
        self.should_run = should_run;
    }
}