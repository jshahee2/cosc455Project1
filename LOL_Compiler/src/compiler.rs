use std::collections::HashMap;

use crate::traits::Compiler;

pub struct LolCompiler {
    // used by the lexer
    pub chars: Vec<char>, // the whole source file as characters
    pub pos: usize,       // index of the next character to read
    pub line: usize,      // current line number (for error messages)
    pub lexeme: String,   // the token being built right now

    // filled by the lexer, read by the parser
    pub tokens: Vec<String>, // every token in order
    pub lines: Vec<usize>,   // the line number of each token
    pub tok_pos: usize,      // index of the token the parser is looking at
    pub current: String,     // the token the parser is looking at

    // used by the parser
    pub scopes: Vec<HashMap<String, String>>, // one map of variables per open block
    pub html: String,                         // the HTML
}

impl LolCompiler {
    pub fn new() -> LolCompiler {
        LolCompiler {
            chars: Vec::new(),
            pos: 0,
            line: 1,
            lexeme: String::new(),
            tokens: Vec::new(),
            lines: Vec::new(),
            tok_pos: 0,
            current: String::new(),
            scopes: Vec::new(),
            html: String::new(),
        }
    }

    pub fn get_html(&self) -> String {
        self.html.clone()
    }

    /// Run the lexer over the whole source and store every token.
    pub fn load_tokens(&mut self, source: &str) {
        self.chars = source.chars().collect();
        self.pos = 0;
        self.line = 1;
        self.tokens.clear();
        self.lines.clear();
        self.html.clear();
        self.scopes.clear();
        loop {
            let tok = self.next_token();
            if tok.is_empty() {
                break;
            }
            self.tokens.push(tok);
            self.lines.push(self.line);
        }
    }
}

impl Compiler for LolCompiler {
    fn compile(&mut self, source: &str) {
        // IMPLEMENTTTTTTTTTTTTTTTT
    }

    fn next_token(&mut self) -> String {
        // ALSO IMPLEMENTTTTTTTTTTT
    }

    fn parse(&mut self) {
        // ALSO ALSOOOO IMPLEMENTTTTTTTTTTTTTTT
    }

    fn current_token(&self) -> String {
        self.current.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current = tok;
    }
}
