#[derive(Debug)]
pub enum ParseError { // erreurs liées aux entrées utilisateurs 
    UnknownCommand(String),
    EmptyInput,
    MissingArgument(String),
}

#[derive(Debug)]
pub enum GameError { // erreurs liées au jeu
    InvalidChoice(String),
    MissingItem(String),
    CommandError(String),
}