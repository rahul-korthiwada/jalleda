use std::fs::File;
use std::io::{Read, BufReader};
use std::thread::sleep;
use std::time::Duration;

pub fn read_contents_from_file(reader: &mut BufReader<File>) -> Option<std::string::String> {
    
    let mut buffer = [0; 1024]; // Adjust the buffer size as needed

    match reader.read(&mut buffer) {
        Ok(0) => {
            // End of file reached, sleep or take other actions
            // sleep(Duration::from_secs(1)); // Adjust as needed
            None
            
        }
        Ok(bytes_read) => {
            // Process the chunk of data
            // 'buffer' contains 'bytes_read' valid bytes
            Some(String::from_utf8_lossy(&buffer[0..bytes_read]).into_owned())
        }
        Err(err) => {
            // Handle the error, e.g., print it
            eprintln!("Error reading from file: {}", err);
            None
        }
    }

}