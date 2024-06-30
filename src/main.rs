use std::collections::HashMap;
static TOKEN_COMMA: &str = ",";
static TOKEN_CLOSED_BRACKET: &str = "]";
static TOKEN_OPEN_BRACKET: &str = "[";
static TOKEN_OPEN_CURLY: &str = "{";
static TOKEN_CLOSED_CURLY: &str = "}";
static TOKEN_DOUBLE_QUOTE: &str = "\"";
static TOKEN_COLON: &str = ":";
static NULL_BOOL: [&str; 3] = ["false", "true", "null"];
fn main() {
    let mut p = Parser::new();
    p.parse(String::from(
        "[1,2,3, {\"key\" : [1,2,344,56,{\"key\" : []}]}]",
    ));
    println!("{:?}", p.get(vec!["3", "key", "4"]));
}
#[derive(Debug, Clone)]
enum JSON {
    STRING(String),
    NUMBER(f64), // ?
    BOOLEAN(bool),
    NULL,
    RECORD(Box<HashMap<String, JSON>>),
    ARRAY(Box<Vec<JSON>>),
}

fn tokenizer(str: String) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    for t in 0..str.len() {}
    return Vec::new();
}

struct Parser {
    ast: JSON,
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            ast: JSON::NULL,
            tokenizer: Tokenizer::new(),
        }
    }
    fn parse(&mut self, v: String) {
        self.tokenizer.tokenizer(v);
        self.start();
    }
    fn start(&mut self) {
        if self
            .tokenizer
            .is_token(&TokenType::as_char(TokenType::CurlyOpenBracket))
        {
            let mut hm_box: Box<HashMap<String, JSON>> = Box::new(HashMap::new());
            self.RS(&mut hm_box);
            self.ast = JSON::RECORD(hm_box);
        } else if self
            .tokenizer
            .is_token(&TokenType::as_char(TokenType::OpenBracket))
        {
            let mut arr_box = Box::new(Vec::new());
            self.ARR_S(&mut arr_box);
            self.ast = JSON::ARRAY(arr_box);
        }
    }

    fn get(&self, keys: Vec<&'static str>) -> Option<JSON> {
        let mut k: &JSON = &self.ast.clone();
        for i in keys.iter() {
            let s = (*i).to_string();
            match k {
                JSON::ARRAY(ref arr) => {
                    k = arr
                        .get(self.tokenizer.parse_number(&s).unwrap() as usize)
                        .unwrap();
                }
                JSON::RECORD(v) => k = v.get(&s).unwrap(),
                _ => {
                    return Some(k.clone());
                }
            }
        }
        return Some(k.clone());
    }

    fn RS(&mut self, hm_box: &mut Box<HashMap<String, JSON>>) {
        self.tokenizer.update(&String::from(TokenType::as_char(
            TokenType::CurlyOpenBracket,
        )));
        self.RC(hm_box);
        self.tokenizer.update(&String::from(TokenType::as_char(
            TokenType::CurlyCloseBracket,
        )));
    }

    fn RC(&mut self, hm_box: &mut Box<HashMap<String, JSON>>) {
        if self
            .tokenizer
            .is_token(&String::from(TokenType::as_char(TokenType::DoubleQuote)))
        {
            self.RA(hm_box);
        }
    }

    fn RM(&mut self, hm_box: &mut Box<HashMap<String, JSON>>) {
        if self.tokenizer.is_token(&String::from("{")) {
            self.RS(hm_box);
        }
    }

    pub fn RA(&mut self, hm_box: &mut Box<HashMap<String, JSON>>) {
        self.RK(hm_box);
        if self.tokenizer.is_token(&String::from(",")) {
            self.tokenizer.update(&String::from(","));
            self.RA(hm_box);
        }
    }
    fn RK(&mut self, hm_box: &mut Box<HashMap<String, JSON>>) {
        self.tokenizer
            .update(&String::from(TokenType::as_char(TokenType::DoubleQuote)));
        let k = self.tokenizer.get_token().clone();
        self.tokenizer.update(&k);
        self.tokenizer
            .update(&String::from(TokenType::as_char(TokenType::DoubleQuote)));
        self.tokenizer
            .update(&String::from(TokenType::as_char(TokenType::Colon)));
        self.RV(&k, hm_box);
    }

    fn RV(&mut self, k: &String, hm_box: &mut Box<HashMap<String, JSON>>) {
        let token = self.tokenizer.get_token().clone();
        match self.tokenizer.parse_number(&token) {
            Some(n) => {
                hm_box.insert(k.clone(), JSON::NUMBER(n));
                self.tokenizer.update(&token);
            }
            None => {
                if NULL_BOOL.contains(&token.as_str()) {
                    let hm_val = self.tokenizer.get_token();
                    hm_box.insert(k.clone(), self.tokenizer.parse_null_bool(hm_val.to_string()));
                    self.tokenizer.update(&hm_val.clone());
                } else if self
                    .tokenizer
                    .is_token(&String::from(TokenType::as_char(TokenType::DoubleQuote)))
                {
                    self.tokenizer
                        .update(&String::from(TokenType::as_char(TokenType::DoubleQuote)));
                    let hm_val = self.tokenizer.get_token();
                    hm_box.insert(k.clone(), JSON::STRING(hm_val.clone()));
                    self.tokenizer.update(&hm_val.clone());
                    self.tokenizer
                        .update(&String::from(TokenType::as_char(TokenType::DoubleQuote)));
                } else if self
                    .tokenizer
                    .is_token(&TokenType::as_char(TokenType::CurlyOpenBracket))
                {
                    hm_box.insert(k.clone(), JSON::RECORD(Box::new(HashMap::new())));
                    match hm_box.get_mut(&k.clone()).unwrap() {
                        JSON::RECORD(hm) => self.RM(hm),
                        _ => {}
                    }
                } else if self
                    .tokenizer
                    .is_token(&TokenType::as_char(TokenType::OpenBracket))
                {
                    hm_box.insert(k.clone(), JSON::ARRAY(Box::new(Vec::new())));
                    match hm_box.get_mut(&k.clone()).unwrap() {
                        JSON::ARRAY(hm) => self.ARR_S(hm),
                        _ => {}
                    }
                } else {
                    panic!(
                        "Expected array, number, string or object but found {}",
                        token
                    );
                }
            }
        }
    }

    fn ARR_S(&mut self, arr_box: &mut Box<Vec<JSON>>) {
        self.tokenizer
            .update(&TokenType::as_char(TokenType::OpenBracket));
        self.ARR_COMMA(arr_box);
        self.tokenizer
            .update(&TokenType::as_char(TokenType::CloseBracket));
    }
    fn ARR_COMMA(&mut self, arr_box: &mut Box<Vec<JSON>>) {
        if !self
            .tokenizer
            .is_token(&TokenType::as_char(TokenType::CloseBracket))
        {
            self.ARR_RV(arr_box);
            if self
                .tokenizer
                .is_token(&TokenType::as_char(TokenType::Comma))
            {
                self.tokenizer.update(&TokenType::as_char(TokenType::Comma));
                self.ARR_COMMA(arr_box);
            }
        }
    }

    fn ARR_RV(&mut self, arr_box: &mut Box<Vec<JSON>>) {
        let token = self.tokenizer.get_token().clone();
        match self.tokenizer.parse_number(&token) {
            Some(n) => {
                arr_box.push(JSON::NUMBER(n));
                self.tokenizer.update(&token);
            }
            None => {
                if NULL_BOOL.contains(&token.as_str()) {
                    let hm_val = self.tokenizer.get_token();
                    arr_box.push(self.tokenizer.parse_null_bool(hm_val.to_string()));
                    self.tokenizer.update(&hm_val.clone());
                } else if self
                    .tokenizer
                    .is_token(&String::from(TokenType::as_char(TokenType::DoubleQuote)))
                {
                    self.tokenizer
                        .update(&String::from(TokenType::as_char(TokenType::DoubleQuote)));
                    let hm_val = self.tokenizer.get_token();
                    arr_box.push(JSON::STRING(hm_val.to_string()));
                    self.tokenizer.update(&hm_val.clone());
                    self.tokenizer
                        .update(&String::from(TokenType::as_char(TokenType::DoubleQuote)));
                } else if self
                    .tokenizer
                    .is_token(&TokenType::as_char(TokenType::CurlyOpenBracket))
                {
                    arr_box.push(JSON::RECORD(Box::new(HashMap::new())));
                    match arr_box.last_mut().unwrap() {
                        JSON::RECORD(v) => self.RM(v),
                        _ => {}
                    }
                } else if self
                    .tokenizer
                    .is_token(&TokenType::as_char(TokenType::OpenBracket))
                {
                    arr_box.push(JSON::ARRAY(Box::new(Vec::new())));
                    match arr_box.last_mut().unwrap() {
                        JSON::ARRAY(v) => self.ARR_S(v),
                        _ => {}
                    }
                } else {
                    panic!(
                        "Expected array, number, string or object but found {}",
                        token
                    );
                }
            }
        }
    }
}

struct Tokenizer {
    pub tokens: Vec<String>,
    counter: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            tokens: Vec::new(),
            counter: 0,
        }
    }
    pub fn update(&mut self, t: &String) {
        match self.tokens.get(self.counter) {
            None => {
                panic!("Token with counter index {} not found", self.counter);
            }
            Some(current_token) => {
                if t != current_token {
                    panic!(
                        "Expected = {} Found = {} Counter = {}",
                        t, current_token, self.counter
                    );
                }
                self.counter += 1;
            }
        }
    }
    pub fn parse_null_bool(&self, t: String) -> JSON {
        match t.as_str() {
            "false" => JSON::BOOLEAN(false),
            "true" => JSON::BOOLEAN(true),
            _ => JSON::NULL,
        }
    }
    pub fn parse_number(&self, t: &String) -> Option<f64> {
        match t.parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
    pub fn get_token(&self) -> &String {
        return &self.tokens[self.counter];
    }
    pub fn is_token(&self, t: &String) -> bool {
        return self.get_token() == t;
    }
    pub fn tokenizer(&mut self, val: String) -> &mut Self {
        let mut current: String = "".to_string();
        let mut is_quote: bool = false;

        for s in val.split("").into_iter() {
            if [TOKEN_DOUBLE_QUOTE].contains(&s) && !is_quote {
                is_quote = true;
                self.tokens.push(s.to_string());
                continue;
            }
            if !(is_quote && s != "\"") {
                if [" ", "\n"].contains(&s) {
                    if current.len() > 0 {
                        self.tokens.push(current);
                        current = "".to_string();
                    }
                    continue;
                }
                if [
                    TOKEN_COMMA,
                    TOKEN_CLOSED_BRACKET,
                    TOKEN_OPEN_BRACKET,
                    TOKEN_OPEN_CURLY,
                    TOKEN_CLOSED_CURLY,
                    TOKEN_DOUBLE_QUOTE,
                    TOKEN_COLON,
                ]
                .contains(&s)
                {
                    if current.len() > 0 {
                        self.tokens.push(current);
                    }
                    if s == TOKEN_DOUBLE_QUOTE {
                        is_quote = false;
                    }
                    current = "".to_string();
                    self.tokens.push(s.to_string());
                    continue;
                }
            }
            current += s;
        }
        self
    }
}

#[derive(Debug)]
enum TokenType {
    OpenBracket,
    CloseBracket,
    CurlyOpenBracket,
    CurlyCloseBracket,
    DoubleQuote,
    Comma,
    Colon,
}

impl TokenType {
    fn as_char(t: TokenType) -> String {
        match t {
            TokenType::OpenBracket => TOKEN_OPEN_BRACKET.to_string(),
            TokenType::CloseBracket => TOKEN_CLOSED_BRACKET.to_string(),
            TokenType::CurlyOpenBracket => TOKEN_OPEN_CURLY.to_string(),
            TokenType::CurlyCloseBracket => TOKEN_CLOSED_CURLY.to_string(),
            TokenType::DoubleQuote => TOKEN_DOUBLE_QUOTE.to_string(),
            TokenType::Comma => TOKEN_COMMA.to_string(),
            TokenType::Colon => TOKEN_COLON.to_string(),
        }
    }
}
