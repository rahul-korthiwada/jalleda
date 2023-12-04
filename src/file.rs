use std::fs::File;
use std::io::{Read, BufReader};
use std::thread::sleep;
use std::time::Duration;
use serde_json;
use crate::app::DefaultLogLevel;

pub fn parse_strings_json(parsed_strings: &mut Vec<String>) -> Vec<serde_json::Value> {
    let mut json_values : Vec<serde_json::Value> = Vec::new();
    for json_str in parsed_strings {
        match serde_json::from_str(json_str) {
            Ok(json_val) => {
                json_values.push(json_val)
            }
            Err(_) => {}
        }
    }
    json_values
}

pub fn match_log_level(json_val : &mut serde_json::Value, log_level:DefaultLogLevel) -> bool {
    let debug_category = json_val["level"].as_str();
    let mut log_debug_category = None;
    match debug_category {
        Some(val) => {
            if val == "Debug" {
                log_debug_category = Some(DefaultLogLevel::DEBUG);
            }
            else if val == "Info" {
                log_debug_category = Some(DefaultLogLevel::INFO);
            }
            else if val == "Warning" {
                log_debug_category = Some(DefaultLogLevel::WARNING);
                
            }
            else if val == "Error" {
                log_debug_category = Some(DefaultLogLevel::ERROR);
            } 
            else {}
        }
        None => {}
    }
    match log_debug_category {
        Some(val) => {
            val == log_level
        }
        None => {
            true
        }
    }
}


pub fn read_contents_from_file(reader: &mut BufReader<File>, temp_buffer: &mut Vec<u8>) -> Vec<String> {
    
    let mut buffer = [0; 1024]; // Adjust the buffer size as needed
    // let mut temp_buffer: Vec<u8> = Vec::new(); 
    let mut parsed_strings: Vec<String> = Vec::new();


    loop{
        match reader.read(&mut buffer) {
            Ok(0) => {
                // End of file reached, sleep or take other actions
                // sleep(Duration::from_secs(1)); // Adjust as needed
                break;
                
            }
            Ok(bytes_read) => {
                Some(String::from_utf8_lossy(&buffer[0..bytes_read]).into_owned());
                for i in 0..bytes_read {
                    if buffer[i] == b'\n' {
                        let parsed_string = String::from_utf8_lossy(&temp_buffer).into_owned();
                        temp_buffer.clear();
                        parsed_strings.push(parsed_string);
                    }
                    else{
                        temp_buffer.push(buffer[i]);
                    }
                }
                ()
            }
            Err(err) => {
                // Handle the error, e.g., print it
                eprintln!("Error reading from file: {}", err);
                break;
            }
        }
    }
    parsed_strings

}