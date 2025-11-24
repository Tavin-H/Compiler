
use regex::Regex;
fn main() {
    let void = Regex::new(r"^(?<match>([ ]+)?void)").unwrap();
    let constant = Regex::new(r"^(?<match>([ ]+)?[0-9]+)").unwrap();
    let identifier = Regex::new(r"^(?<match>([ ]+)?[a-zA-Z]+)").unwrap();
    let first_word_capture = Regex::new(r"^(?<match>[ ]+?[0-9]+?[a-zA-Z]+?)").unwrap();
    let mut test_string = String::from("  123 ))  hi  (){");
    fn remove_from_string(target_string: &mut String, string_to_remove: &str) {
        let length: usize = string_to_remove.len() as usize;
        target_string.replace_range(..length, "");
        return;
    }
    fn Tokenize(string: &mut String) {
        let void = Regex::new(r"^(?<match>([ ]+)?void)\b").unwrap();
        let constant = Regex::new(r"^(?<match>([ ]+)?[0-9]+)\b").unwrap();
        let identifier = Regex::new(r"^(?<match>([ ]+)?[a-zA-Z]+)\b").unwrap();
        let returnKeyword = Regex::new(r"^(?<match>([ ]+)?return)\b").unwrap();
        let closingParenthesis = Regex::new(r"^(?<match>([ ]+)?\))").unwrap();
        let openingParenthesis = Regex::new(r"^(?<match>([ ]+)?\()").unwrap();
        let openingBrace = Regex::new(r"^(?<match>([ ]+)?\{)").unwrap();
        let closingBrace = Regex::new(r"^(?<match>([ ]+)?\})").unwrap();
        let semicolon = Regex::new(r"^(?<match>([ ]+)?;)").unwrap();
        let mut capture: String = String::new();
        if constant.is_match(string) {
            let Some(cap) = constant.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            println!("Token: Const");
        } else if closingParenthesis.is_match(string) {
            let Some(cap) = closingParenthesis.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            println!("Token: closing parenthesis")
        } else if openingParenthesis.is_match(string) {
            let Some(cap) = openingParenthesis.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            println!("Token: opening parenthesis");
        } else if openingBrace.is_match(string) {
            let Some(cap) = openingBrace.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            println!("Token: opening brace");
        } else if openingParenthesis.is_match(string) {
            let Some(cap) = openingParenthesis.captures(string) else {
                return;
            };
            capture = cap["match"].to_string();
            println!("Token: opening parenthesis");
        } else if identifier.is_match(string) {
            let Some(cap) = identifier.captures(string) else {
                return;
            };
            if void.is_match(&cap["match"]) {
                println!("might be a keyword, checkin: {}", &cap["match"]);
                let Some(cap2) = void.captures(string) else {
                    return;
                };
                capture = cap2["match"].to_string();
                println!("Token: Void");
            }
            capture = cap["match"].to_string();
            println!("Token: identifier");
        } else {
            panic!("ERROR - No tokens match substring {}", string);
        }
        remove_from_string(string, &capture);
    }
    while !test_string.is_empty() {
        println!("code: {}", test_string);
        Tokenize(&mut test_string);
    }
}
