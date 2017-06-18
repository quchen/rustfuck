pub enum Token {
    Add,
    Sub,
    Left,
    Right,
    LoopL,
    LoopR,
    Read,
    Print,
}

pub fn tokenize(input: String) -> Vec<Token> {
    // Conservatively allocate the input byte length
    let mut result: Vec<Token> = Vec::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '+' => result.push(Token::Add),
            '-' => result.push(Token::Sub),
            '<' => result.push(Token::Left),
            '>' => result.push(Token::Right),
            '[' => result.push(Token::LoopL),
            ']' => result.push(Token::LoopR),
            ',' => result.push(Token::Read),
            '.' => result.push(Token::Print),
            _   => {},
        }
    }
    result.shrink_to_fit();
    result
}
