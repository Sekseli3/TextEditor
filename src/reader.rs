use crossterm::{cursor, event::read, QueueableCommand};

use std::io::{stdout, Write};
pub struct State {
    pub normal: String,
    pub insert: String,
}
fn handle_insert(char: char, cx: &mut u16, buffer: &mut String) {
    buffer.push(char);
    std::write!(stdout(), "{}", char).unwrap();
    *cx += 1;
}
fn move_cursor(cx: &mut u16, cy: &mut u16, direction: char) {
    match direction {
        'h' => {
            if *cx > 0 {
                *cx -= 1
            } else {
                *cx = 0
            }
        }
        'j' => *cy += 1,
        'k' => {
            if *cy > 0 {
                *cy -= 1
            } else {
                *cy = 0
            }
        }
        'l' => *cx += 1,
        _ => {}
    }
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
                crossterm::event::KeyCode::Char(':') => break,
                crossterm::event::KeyCode::Char('i') => {
                    if current_state == &state.normal {
                        current_state = &state.insert;
                    } else {
                        handle_insert('i', &mut cx, &mut buffer);
                    }
                }
                crossterm::event::KeyCode::Esc => {
                    if current_state == &state.insert {
                        current_state = &state.normal;
                    }
                }
                crossterm::event::KeyCode::Char('h') => {
                    if current_state != &state.insert {
                        move_cursor(&mut cx, &mut cy, 'h');
                    } else {
                        handle_insert('h', &mut cx, &mut buffer);
                    }
                }
                crossterm::event::KeyCode::Char('j') => {
                    if current_state != &state.insert {
                        move_cursor(&mut cx, &mut cy, 'j');
                    } else {
                        handle_insert('j', &mut cx, &mut buffer);
                    }
                }
                crossterm::event::KeyCode::Char('k') => {
                    if current_state != &state.insert {
                        move_cursor(&mut cx, &mut cy, 'k');
                    } else {
                        handle_insert('k', &mut cx, &mut buffer);
                    }
                }
                crossterm::event::KeyCode::Char('l') => {
                    if current_state != &state.insert {
                        move_cursor(&mut cx, &mut cy, 'l');
                    } else {
                        handle_insert('l', &mut cx, &mut buffer);
                    }
                }
                crossterm::event::KeyCode::Char(c) => {
                    if current_state == &state.insert {
                        handle_insert(c, &mut cx, &mut buffer);
                    }
                }

                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
