pub type Result<R> = std::result::Result<R, (Error, u32)>;

pub enum Error {
    NumberAlreadyDecimal,

    MissingParentheses,
    MissingBraces,
    MissingColon,

    MisplacedParentheses,

    UnclosedQuotes,
    UnclosedParentheses,
    UnclosedBraces,

    UnrecognizableCharacter,
    UnrecognizableToken,
    UnexpectedToken,

    VariableInvalidName,
    VariableInvalidType,
    VariableMissingEquals,

    FunctionInvalidName,
    FunctionInvalidType,
    FunctionArgumentMissingName,
    FunctionArgumentMissingType,

    Other,
}

impl Error {
    pub fn handle_error(&self, line: u32, origin: &str) {
        let message: &str = match self {
            Error::NumberAlreadyDecimal => "This number is already floating-point",
            Error::MissingParentheses => "Missing parentheses",
            Error::MissingBraces => "Missing braces",
            Error::MissingColon => "Missing colon to separe an identifier from it's type",

            Error::MisplacedParentheses => "Misplaced parentheses",

            Error::UnclosedQuotes => "Unclosed quotes",
            Error::UnclosedParentheses => "Unclosed parentheses",
            Error::UnclosedBraces => "Unclosed braces",

            Error::UnrecognizableCharacter => "Unrecognizable character",
            Error::UnrecognizableToken => "Unrecognizable token",
            Error::UnexpectedToken => "Unexpected token",

            Error::VariableInvalidName => "Invalid name for variable declaration",
            Error::VariableInvalidType => "Invalid type for variable declaration",
            Error::VariableMissingEquals => "Missing \'=\' sign for variable's value declaration",

            Error::FunctionInvalidName => "Invalid name for function declaration",
            Error::FunctionInvalidType => "Invalid return type for function declaration",
            Error::FunctionArgumentMissingName => "Missing name for function argument",
            Error::FunctionArgumentMissingType => "Missing type for function argument",

            Error::Other => panic!("you should have used it at panic function!"),
        };

        let formatted_message = format!("{origin}: {message} at line {line}");
        self.panic(formatted_message);
    }

    pub fn panic(&self, msg: String) {
        eprintln!("ERROR: {msg}");
        std::process::exit(1);
    }
}
