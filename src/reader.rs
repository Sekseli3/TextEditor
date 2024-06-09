use crossterm::{cursor, event::read, QueueableCommand};

use std::io::{stdout, Write};
pub struct State {
    pub normal: String,
    pub insert: String,
}
pub fn handle_input(state: State) -> anyhow::Result<()> {
    let mut current_state = &state.normal; // default state
    let mut cx = 0;
    let mut cy = 0;
    let mut buffer = String::new();
    let mut stdout = stdout();
    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;
        match read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char('q') => break,
                crossterm::event::KeyCode::Char('i') => {
                    if current_state == &state.normal {
                        current_state = &state.insert;
                    }
                }
                crossterm::event::KeyCode::Char('a') => {
                    if current_state == &state.insert {
                        current_state = &state.normal;
                    }
                }
                crossterm::event::KeyCode::Char('h') => {
                    if current_state != &state.insert {
                        if cx > 0 {
                            cx -= 1
                        } else {
                            cx = 0
                        }
                    }
                }
                crossterm::event::KeyCode::Char('j') => {
                    if current_state != &state.insert {
                        cy += 1
                    }
                }
                crossterm::event::KeyCode::Char('k') => {
                    if current_state != &state.insert {
                        if cy > 0 {
                            cy -= 1
                        } else {
                            cy = 0
                        }
                    }
                }
                crossterm::event::KeyCode::Char('l') => {
                    if current_state != &state.insert {
                        cx += 1
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    if current_state == &state.insert {
                        buffer.push(c);
                        std::write!(stdout, "{}", c)?;
                        cx += 1;
                    }
                }

                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
