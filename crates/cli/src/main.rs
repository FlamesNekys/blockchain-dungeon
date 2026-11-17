use cli::parse_command;
use game::{GameState, apply, render};

fn main() -> std::io::Result<()> {
    let mut state = GameState::genesis();
    print!("{}", render(&state));
    for line in std::io::stdin().lines() {
        let line = line?;
        if line.trim() == "quit" {
            break;
        }
        match parse_command(&line) {
            Ok(command) => {
                let res = apply(&mut state, command.who, command.action);
                if let Ok(events) = res {
                    for event in events {
                        println!("событие: {event:?}");
                    }
                } else if let Err(error) = res {
                    println!("отказ: {error}");
                }
            }
            Err(error) => println!("не понял команду: {error}"),
        }
        print!("{}", render(&state));
    }
    Ok(())
}
