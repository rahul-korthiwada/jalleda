// ANCHOR: all
use std::{error::Error, io};
use std::fs::File;
use std::io::{BufReader, BufRead};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
        KeyEventKind,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
mod file;
use crate::{
    app::{App, CurrentScreen, DefaultLogLevel},
    ui::ui,
    file:: {read_contents_from_file, parse_strings_json}
};

// ANCHOR: main_all
// ANCHOR: setup_boilerplate
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let file = File::open("log.txt")?;
    let mut reader = io::BufReader::new(file);
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    // ANCHOR_END: setup_boilerplate
    // ANCHOR: application_startup
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut reader, &mut app);

    // ANCHOR_END: application_startup

    // ANCHOR: ending_boilerplate
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // ANCHOR_END: ending_boilerplate

    // ANCHOR: final_print
    // if let Ok(do_print) = res {
    //     if do_print {
    //         app.print_json()?;
    //     }
    // } else if let Err(err) = res {
    //     println!("{err:?}");
    // }

    Ok(())
}
// ANCHOR_END: final_print
// ANCHOR_END: main_all

// ANCHOR: run_app_all
// ANCHOR: run_method_signature
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    reader: &mut BufReader<File>,
    app: &mut App,
) -> io::Result<bool> {
    // ANCHOR_END: run_method_signature
    // ANCHOR: ui_loop
    let mut temp_buffer: Vec<u8> = Vec::new();
    loop {
        terminal.draw(|f| ui(f, app))?;
        let mut content = read_contents_from_file(reader, &mut temp_buffer);
        app.logs.extend(parse_strings_json(&mut content));
        // ANCHOR_END: ui_loop

        // ANCHOR: event_poll
        // ANCHOR: main_screen
        if crossterm::event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('D') => {
                            app.current_screen = CurrentScreen::DefaultFiltering(DefaultLogLevel::DEBUG);
                        }
                        KeyCode::Char('I') => {
                            app.current_screen = CurrentScreen::DefaultFiltering(DefaultLogLevel::INFO);
                        }
                        KeyCode::Char('W') => {
                            app.current_screen = CurrentScreen::DefaultFiltering(DefaultLogLevel::WARNING);
                        }
                        KeyCode::Char('E') => {
                            app.current_screen = CurrentScreen::DefaultFiltering(DefaultLogLevel::ERROR);
                        }
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => {}
                    },
                    CurrentScreen::DefaultFiltering(_) => match key.code {
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        _ => {}
                    }
                    // ANCHOR_END: main_screen
                    // ANCHOR: exiting_screen
                //     CurrentScreen::Exiting => match key.code {
                //         KeyCode::Char('y') => {
                //             return Ok(true);
                //         }
                //         KeyCode::Char('n') | KeyCode::Char('q') => {
                //             return Ok(false);
                //         }
                //         _ => {}
                //     },
                //     // ANCHOR_END: exiting_screen
                //     // ANCHOR: editing_enter
                //     CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                //         match key.code {
                //             KeyCode::Enter => {
                //                 if let Some(editing) = &app.currently_editing {
                //                     match editing {
                //                         CurrentlyEditing::Key => {
                //                             app.currently_editing =
                //                                 Some(CurrentlyEditing::Value);
                //                         }
                //                         CurrentlyEditing::Value => {
                //                             app.save_key_value();
                //                             app.current_screen =
                //                                 CurrentScreen::Main;
                //                         }
                //                     }
                //                 }
                //             }
                //             // ANCHOR_END: editing_enter
                //             // ANCHOR: backspace_editing
                //             KeyCode::Backspace => {
                //                 if let Some(editing) = &app.currently_editing {
                //                     match editing {
                //                         CurrentlyEditing::Key => {
                //                             app.key_input.pop();
                //                         }
                //                         CurrentlyEditing::Value => {
                //                             app.value_input.pop();
                //                         }
                //                     }
                //                 }
                //             }
                //             // ANCHOR_END: backspace_editing
                //             // ANCHOR: escape_editing
                //             KeyCode::Esc => {
                //                 app.current_screen = CurrentScreen::Main;
                //                 app.currently_editing = None;
                //             }
                //             // ANCHOR_END: escape_editing
                //             // ANCHOR: tab_editing
                //             KeyCode::Tab => {
                //                 app.toggle_editing();
                //             }
                //             // ANCHOR_END: tab_editing
                //             // ANCHOR: character_editing
                //             KeyCode::Char(value) => {
                //                 if let Some(editing) = &app.currently_editing {
                //                     match editing {
                //                         CurrentlyEditing::Key => {
                //                             app.key_input.push(value);
                //                         }
                //                         CurrentlyEditing::Value => {
                //                             app.value_input.push(value);
                //                         }
                //                     }
                //                 }
                //             }
                //             // ANCHOR_END: character_editing
                //             _ => {}
                //         }
                //     }
                //     _ => {}
                }
        }
    }
        // ANCHOR_END: event_poll
    }
}
// ANCHOR: run_app_all

// ANCHOR_END: all