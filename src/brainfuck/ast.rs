use brainfuck::token::Token;

pub struct Ast(pub Vec<Expr>);
pub enum Expr {
    Add,
    Sub,
    Left,
    Right,
    Loop(Box<Ast>),
    Read,
    Print,
}

struct Parser {
    input: Vec<Token>,
}

// pub fn parse(input: Vec<Token>) -> Option<Ast> {
//     parseIter(input.iter());
// }
//
// fn parseIter(iterator: Iter<Token>) -> Option<Ast> {
//     let mut result = Vec::new();
//     while let Some(tok) = iterator.next() {
//         match *tok {
//             Token::Add => result.push(Expr::Add),
//             Token::Sub => result.push(Expr::Sub),
//             Token::Left => result.push(Expr::Left),
//             Token::Right => result.push(Expr::Right),
//             Token::Read => result.push(Expr::Read),
//             Token::Print => result.push(Expr::Print),
//             Token::LoopL => {
//                 let body = parseIter(
//                 result.push(Box::new(body);
//             },
//             Token::LoopR => return None,
//         }
//     }
//     Some(result)
// }
