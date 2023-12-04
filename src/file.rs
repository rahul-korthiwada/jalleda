use std::fs::File;
use std::io::{Read, BufReader};
use std::thread::sleep;
use std::time::Duration;



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
                // Process the chunk of data
                // 'buffer' contains 'bytes_read' valid bytes
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