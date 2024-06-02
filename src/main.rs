use std::io::{stdout, Write};

use crossterm::{cursor, event::read, terminal, ExecutableCommand, QueueableCommand};
// this gives error match arms have imcompitable types, fix it
fn main() -> anyhow::Result<()> {
    let mut cx = 0;
    let mut cy = 0;
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen);
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    loop {
        stdout.queue(cursor::MoveTo(cx, cy))?;
        stdout.flush()?;
        match read()? {
            crossterm::event::Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Char('q') => break,
                //Fix this error:
                crossterm::event::KeyCode::Char('h') => {
                    if cx > 0 {
                        cx -= 1
                    } else {
                        cx = 0
                    }
                }
                crossterm::event::KeyCode::Char('j') => cy += 1,
                crossterm::event::KeyCode::Char('k') => {
                    if cy > 0 {
                        cy -= 1
                    } else {
                        cy = 0
                    }
                }
                crossterm::event::KeyCode::Char('l') => cx += 1,

                _ => {}
            },
            _ => {}
        }
    }
    stdout.execute(terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode()?;

    Ok(())
}
