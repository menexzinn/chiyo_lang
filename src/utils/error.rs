pub enum Error {
    UnclosedQuotes,
    UnclosedParentheses,

    UnrecognizableCharacter,
    UnrecognizableToken,

    VariableInvalidName,
    VariableMissingColon,
    VariableMissingType,
    VariableMissingEquals,
}

pub fn handle_error(error: Error, line: u32) {
    let message: &str = match error {
        Error::UnclosedQuotes => "Unclosed quotes",
        Error::UnclosedParentheses => "Unclosed parentheses",

        Error::UnrecognizableCharacter => "Unrecognizable character",
        Error::UnrecognizableToken => "Unrecognizable token",

        Error::VariableInvalidName => "Invalid name for variable declaration",
        Error::VariableMissingColon => "Missing colon to separe the variable from its type",
        Error::VariableMissingType => "Missing variable's type for its declaration",
        Error::VariableMissingEquals => "Missing \'=\' sign for variable's value declaration",
    };

    panic!("Error: {message} at line {line}");
}
