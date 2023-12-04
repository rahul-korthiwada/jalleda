use std::collections::HashMap;


// ANCHOR: all
// ANCHOR: imports
use serde_json::Result;
use std::io::{BufReader, BufRead};
// ANCHOR_END: imports

// ANCHOR: screen_modes
pub enum CurrentScreen {
    Main,
    DefaultFiltering
}
// ANCHOR_END: screen_modes

// ANCHOR: app_fields
pub struct App {
    pub key_input: String, // the currently being edited json key.
    pub value_input: String, 
    // pub reader: BufReader<File>,// the currently being edited json value.
    pub logs: Vec<String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
}
// ANCHOR_END: app_fields

// ANCHOR: impl_new
impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            logs: Vec::new(),
            current_screen: CurrentScreen::Main,
        }
    }
    // ANCHOR_END: impl_new

    // ANCHOR: save_key_value
    // pub fn save_key_value(&mut self) {
    //     self.pairs
    //         .insert(self.key_input.clone(), self.value_input.clone());

    //     self.key_input = String::new();
    //     self.value_input = String::new();
    //     self.currently_editing = None;
    // }
    // ANCHOR_END: save_key_value

    // ANCHOR: toggle_editing
    // pub fn toggle_editing(&mut self) {
    //     if let Some(edit_mode) = &self.currently_editing {
    //         match edit_mode {
    //             CurrentlyEditing::Key => {
    //                 self.currently_editing = Some(CurrentlyEditing::Value)
    //             }
    //             CurrentlyEditing::Value => {
    //                 self.currently_editing = Some(CurrentlyEditing::Key)
    //             }
    //         };
    //     } else {
    //         self.currently_editing = Some(CurrentlyEditing::Key);
    //     }
    // }
    // ANCHOR_END: toggle_editing

    // ANCHOR: print_json
    // pub fn print_json(&self) -> Result<()> {
    //     let output = serde_json::to_string(&self.pairs)?;
    //     println!("{}", output);
    //     Ok(())
    // }
    // ANCHOR_END: print_json
}
// ANCHOR_END: all