use anyhow::Result;
use relua::{State, Value};
use rustyline::{error::ReadlineError, DefaultEditor};

//

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut state = State::new();
    state.set("print", |l: Vec<Value>| {
        let Some((last, list)) = l.split_last() else {
            return Value::Nil;
        };

        for v in list {
            print!("{v}, ");
        }
        println!("{last}");

        Value::Nil
    });

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
