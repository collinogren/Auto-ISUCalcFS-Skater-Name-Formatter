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
    let mut ctx = ClipboardContext::new().unwrap();
    let mut last_content: Option<String> = None;
    let should_run: Arc<RwLock<Run>> = Arc::new(RwLock::new(Run::new(true)));
    check_for_exit(should_run.clone());
    while should_run.read().unwrap().should_run {
        sleep(Duration::from_millis(50));
        if last_content.is_none() {
            last_content = Some(ctx.get_contents().unwrap());
        }
        
        let content = match ctx.get_contents() {
            Ok(contents) => { contents }
            Err(_) => { continue }
        };

        if content.eq(&last_content.clone().unwrap()) {
            continue;
        } else {
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
            }).unwrap();
        }
    }
}

fn format_full_name(name: String) -> Option<String> {
    let name = name.split(",").nth(0);

    if name.is_none() {
        //println!("Input was none");
        return None;
    }

    //println!("{}", name.unwrap());

    let mut name_split = name.unwrap().split(" ");
    let first_name = name_split.nth(0);
    let last_name = name_split.last();

    if first_name.is_none() || last_name.is_none() {
        //println!("Either first or last name is none");
        return None;
    }

    //println!("{}", first_name.unwrap());
    //println!("{}", last_name.unwrap());

    let first_name_first_four = format_individual_name(first_name.unwrap().to_string());
    let last_name_first_four = format_individual_name(last_name.unwrap().to_string());

    //println!("{}{}", last_name_first_four, first_name_first_four);

    //println!("Success");

    return Some(format!("{}{}", last_name_first_four, first_name_first_four));
}

fn format_individual_name(name: String) -> String {
    let mut name_first_four = String::new();
    let mut name_iter = name.chars();
    for _ in 0..4 {
       name_first_four = format!("{}{}", name_first_four, match name_iter.next() {
            None => { break }
            Some(c) => { c }
        });
    }
    
    return name_first_four;
}

fn check_for_exit(should_run: Arc<RwLock<Run>>) {
    thread::spawn(move || {
        let mut check_input = true;
        println!("Auto ISUCalc Skater Name Formatter is running.\nType 'q' and press enter to exit.");
        while check_input {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            if line.to_lowercase().trim().eq("q") {
                check_input = false;
                should_run.write().unwrap().set_should_run(false);
                println!("Program terminated.")
            }
        }
    });
}

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