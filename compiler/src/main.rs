#![allow(warnings)]
use regex::Regex;

//TODO test with info on page 51 in textbook

fn main() {
    //println!("{}", void.is_match(test_string));

    //=======================================================
    //                         LEXER
    //=======================================================
    #[derive(Debug, PartialEq)]
    enum Token {
        Return,
        Void,
        Int,
        Semicolon,
        OpeningBrace,
        ClosingBrace,
        OpeningParenthesis,
        ClosingParenthesis,
        Identifier(String),
        Constant(i32),
    }
    let mut Tokens: Vec<Token> = Vec::new();
    fn remove_from_string(target_string: &mut String, string_to_remove: &str) {
        let length: usize = string_to_remove.len() as usize;
        target_string.replace_range(..length, "");
        return;
    }
    fn Tokenize(string: &mut String, TokenList: &mut Vec<Token>) {
        //-----------------------Regex builders---------------------
        let void = Regex::new(r"^(?<match>([ ]+)?void)\b").unwrap();
        let constant = Regex::new(r"^(?<match>([ ]+)?(?<actual>[0-9]+))\b").unwrap();
        let identifier = Regex::new(r"^(?<match>([ ]+)?(?<actual>[a-zA-Z]+))\b").unwrap();
        let intKeyword = Regex::new(r"^(?<match>([ ]+)?int)\b").unwrap();
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

            //I am the best programmer on earth
            let number = cap["actual"].to_string();
            match number.parse::<i32>() {
                Ok(num) => {
                    println!("ok");
                    TokenList.push(Token::Constant(num));
                }
                _ => println!("Not ok"),
            }
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
            let Some(cap) = semicolon.captures(string) else {
                return;
            };
            TokenList.push(Token::Semicolon);
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
                TokenList.push(Token::Void);
            } else if returnKeyword.is_match(&cap["match"]) {
                let Some(cap2) = returnKeyword.captures(string) else {
                    return;
                };
                TokenList.push(Token::Return);
                //println!("Token: Return");
            } else if intKeyword.is_match(&cap["match"]) {
                //Int Keyword
                TokenList.push(Token::Int);
            } else {
                let value: String = cap["actual"].to_string();
                TokenList.push(Token::Identifier(value));
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
    #[derive(Debug)]
    enum ASTNode {
        Constant(i32),
        Expression(Box<ASTNode>),             //Needs Constant
        Function(Box<ASTNode>, Box<ASTNode>), // Needs Return
        Program(Box<ASTNode>),                // Needs Function
        Return(Box<ASTNode>),                 // Needs Expression
        Identifier(String),
    }

    //-----------Formal grammer rulesets for basic parser------
    //<Program> ::= <Function>
    //<Function> ::= "int" <Identifier> "(" "void" ")" "{" <statement> "}"
    //<Statement> ::= "return" <Exp> ";"
    //<Exp> ::= <Int>
    //<Identifier> ::= ? An identifier token ?
    //<Int> ::= ? A constant token ?
    //Each < > is a funciton and each ?? or "" is a Token

    fn expect(expected: Token, Tokens: &mut Vec<Token>) {
        println!("expected {:?}", expected);
        println!("Tokens: {:?}", Tokens);
        println!("Removed {:?} at expect", Tokens.remove(0));
        println!(" ");

        /*if Tokens[0] != expected {
            panic!(
                "Invalid sytnax! Expected: {:?} but found: {:?}",
                expected, Tokens[0]
            );
        }*/
        //Tokens.remove(0);
    }

    fn parse_int(Tokens: &Vec<Token>) -> ASTNode {
        //expect(Token::Constant(10), Tokens);
        match &Tokens[0] {
            Token::Constant(value) => ASTNode::Constant(*value),
            other => panic!("expected Constant but found {:?}", other),
        }
    }

    fn parse_identifier(Tokens: &mut Vec<Token>) -> ASTNode {
        let temp: Token = Tokens.remove(0); // Rust is weird
        match temp {
            Token::Identifier(value) => ASTNode::Identifier(value),
            other => panic!(
                "Invalid Identifier! expected Identifier and got {:?}",
                other
            ),
        }
    }
    fn parse_expression(Tokens: &mut Vec<Token>) -> ASTNode {
        //println!("token at expression parsing {:?}", Tokens);
        let temp: Token = Tokens.remove(0);

        match temp {
            Token::Constant(value) => ASTNode::Expression(Box::new(ASTNode::Constant(value))),
            other => {
                panic!(
                    "Invalid Expression! Expected: Expression but found {:?}",
                    other
                )
            }
        }
    }
    fn parse_statement(Tokens: &mut Vec<Token>) -> ASTNode {
        expect(Token::Return, Tokens);
        let return_val: Box<ASTNode> = Box::new(parse_expression(Tokens));

        expect(Token::Semicolon, Tokens);
        return ASTNode::Return(return_val);
    }

    fn parse_function(Tokens: &mut Vec<Token>) -> ASTNode {
        expect(Token::Int, Tokens);
        let Identifier: ASTNode = parse_identifier(Tokens);
        expect(Token::OpeningParenthesis, Tokens);
        expect(Token::Void, Tokens);
        expect(Token::ClosingParenthesis, Tokens);
        expect(Token::OpeningBrace, Tokens);
        let Statement: ASTNode = parse_statement(Tokens);
        //println!("Parse function removed: {:?}", Tokens.remove(0));
        expect(Token::ClosingBrace, Tokens);
        ASTNode::Function(Box::new(Identifier), Box::new(Statement))
    }

    fn parse_program(Tokens: &mut Vec<Token>) -> ASTNode {
        let function: ASTNode = parse_function(Tokens);
        return ASTNode::Program(Box::new(function));
    }

    //====================================================
    //                Assembly Generation
    //====================================================
    enum ACon {
        // Assmebly Construct
        Function(String, Vec<Instruction>),
        Operand(Operand),
        Instruction(Instruction),
    }
    enum Operand {
        Register(Register),
        Imm(i32),
    }
    enum Register {
        EAX,
    }
    enum Instruction {
        Mov(Operand, Operand),
        Ret,
    }

    //====================================================
    //                      Testing
    //====================================================
    let mut test_string = String::from("int main(void) { return 2; }");
    println!("code: {}", test_string);
    while !test_string.is_empty() {
        Tokenize(&mut test_string, &mut Tokens);
    }
    println!("Token List {:?}", Tokens);

    println!("{:?}", parse_program(&mut Tokens));
    //   expect(Token::Void, Vec::from([Token::Void]));
}
