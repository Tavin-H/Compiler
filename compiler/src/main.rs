#![allow(warnings)]
use regex::Regex;

//TODO test with info on page 51 in textbook

fn main() {
    //println!("{}", void.is_match(test_string));

    //=======================================================
    //                         LEXER
    //=======================================================
    #[derive(Debug)]
    enum Token {
        Return,
        Void,
        Semicolon,
        OpeningBrace,
        ClosingBrace,
        OpeningParenthesis,
        ClosingParenthesis,
        Identifier,
        Constant,
    }
    let mut Tokens: Vec<Token> = Vec::new();
    fn remove_from_string(target_string: &mut String, string_to_remove: &str) {
        let length: usize = string_to_remove.len() as usize;
        target_string.replace_range(..length, "");
        return;
    }
    fn Tokenize(string: &mut String, TokenList: &mut Vec<Token>) {
        //Regex builders
        let void = Regex::new(r"^(?<match>([ ]+)?void)\b").unwrap();
        let constant = Regex::new(r"^(?<match>([ ]+)?[0-9]+)\b").unwrap();
        let identifier = Regex::new(r"^(?<match>([ ]+)?[a-zA-Z]+)\b").unwrap();
        let returnKeyword = Regex::new(r"^(?<match>([ ]+)?return)\b").unwrap();
        let closingParenthesis = Regex::new(r"^(?<match>([ ]+)?\))").unwrap();
        let openingParenthesis = Regex::new(r"^(?<match>([ ]+)?\()").unwrap();
        let openingBrace = Regex::new(r"^(?<match>([ ]+)?\{)").unwrap();
        let closingBrace = Regex::new(r"^(?<match>([ ]+)?\})").unwrap();
        let semicolon = Regex::new(r"^(?<match>([ ]+)?;)").unwrap();
        let whiteSpace = Regex::new(r"^(?<match>[ ]+)").unwrap();
        //Tokenize
        let mut capture: String = String::new();
        if constant.is_match(string) {
            let Some(cap) = constant.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            TokenList.push(Token::Constant);
            //println!("Token: Const");
        } else if closingParenthesis.is_match(string) {
            let Some(cap) = closingParenthesis.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            TokenList.push(Token::ClosingParenthesis);
            //println!("Token: closing parenthesis")
        } else if openingParenthesis.is_match(string) {
            let Some(cap) = openingParenthesis.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            TokenList.push(Token::OpeningParenthesis);
            //println!("Token: opening parenthesis");
        } else if openingBrace.is_match(string) {
            let Some(cap) = openingBrace.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            TokenList.push(Token::OpeningBrace);
            //println!("Token: opening brace");
        } else if closingBrace.is_match(string) {
            //FIXME
            let Some(cap) = closingBrace.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            TokenList.push(Token::ClosingBrace);
            //println!("Token: closing brace");
        } else if semicolon.is_match(string) {
            //FIXME
            let Some(cap) = semicolon.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            //println!("Token: semicolon");
        } else if identifier.is_match(string) {
            //All keywords will match so check those next
            let Some(cap) = identifier.captures(string) else {
                return;
            };

            if void.is_match(&cap["match"]) {
                let Some(cap2) = void.captures(string) else {
                    return;
                };
                //println!("Token: Void");
            } else if returnKeyword.is_match(&cap["match"]) {
                let Some(cap2) = returnKeyword.captures(string) else {
                    return;
                };
                //println!("Token: Return");
            } else {
                //println!("Token: identifier");
            }
            // Case where all keywords are exhausted
            capture = cap["match"].to_string();
        } else if whiteSpace.is_match(string) {
            let Some(cap) = whiteSpace.captures(string) else {
                return;
            };
            //println!("Just some whiteSpace");
            capture = cap["match"].to_string();
        } else {
            panic!("ERROR - No tokens match substring {}", string);
        };

        remove_from_string(string, &capture);
    }

    //====================================================
    //                      PARSER
    //====================================================

    //Parser code goes here
    // !! Note: This is not type safe because rust doesn't allow recursive enums!!
    enum ASTNode {
        Constant(i32),
        Expression(Box<ASTNode>), //Needs Constant
        Function(Box<ASTNode>),   // Needs Expression
        Program(Box<ASTNode>),    // Needs Function
    }

    //====================================================
    //                      Testing
    //====================================================
    let mut test_string = String::from("void return hi 123 () {} ;  ");
    println!("code: {}", test_string);
    while !test_string.is_empty() {
        //println!("code: {}", test_string);

        Tokenize(&mut test_string, &mut Tokens);
    }
    println!("Token List {:?}", Tokens);
}
