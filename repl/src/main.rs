use anyhow::Result;
use relua::State;
use rustyline::{error::ReadlineError, DefaultEditor};

//

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut state = State::new();

    let mut rl = DefaultEditor::new()?;
    _ = rl.load_history(".repl-history");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if let Err(err) = state.run(line.as_str()) {
                    eprintln!("error: {err}");
                }
                _ = rl.add_history_entry(line);
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("use Ctrl+D to exit");
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("error: {err}");
            }
        }
    }

    _ = rl.save_history(".repl-history");

    Ok(())
}
