enum GameCommand {
    Hello(String),
    Say(String),
    Goodbye(String),
    Quit,
}

fn process_command(command: GameCommand) {
    match command {
        GameCommand::Hello(name) => println!("Hi, {}", name),
        GameCommand::Quit => println!("You quit the game."),
        GameCommand::Goodbye(name) => println!("Goodbye, {}", name),
        GameCommand::Say(message) => println!("You said: {}", message),
    }
}

fn main() {
    let cmd1 = GameCommand::Hello(String::from("Sara"));
    let cmd2 = GameCommand::Quit;
    let cmd3 = GameCommand::Goodbye(String::from("Alice"));
    let cmd4 = GameCommand::Goodbye(String::from("Bob"));
    let vector: Vec<GameCommand> = vec![cmd1, cmd2, cmd3, cmd4];
    for command in vector {
        process_command(command);
    }
}
