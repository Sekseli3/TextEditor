use std::io::stdout;

use crossterm::{terminal, ExecutableCommand};

mod reader;
// this gives error match arms have imcompitable types, fix it
fn main() -> anyhow::Result<()> {
    let state = reader::State {
        normal: "Normal".to_string(),
        insert: "Insert".to_string(),
    };
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen);
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    reader::handle_input(state)?;
    stdout.execute(terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode()?;

    Ok(())
}
