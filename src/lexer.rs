use std::process;

use crate::compiler::LolCompiler;
use crate::traits::LexicalAnalyzer;

/// every valid annotation
const ANNOTATIONS: [&str; 11] = [
    "#HAI", "#KBYE", "#OBTW", "#TLDR", "#MAEK", "#OIC", "#GIMMEH", "#MKAY", "#NEWLINE", "#IHAZ",
    "#ITIZ",
];

/// characters allowed in plain text
fn is_allowed_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || ",.\":?!%=/".contains(c)
}

impl LolCompiler {
    /// prints a lexical error and stop the program
    fn lex_error(&self, msg: &str) -> ! {
        eprintln!("Lexical error at line {}: {}", self.line, msg);
        process::exit(1);
    }

    /// reads a single token
    pub fn read_token(&mut self) -> String {
        self.lexeme.clear();

        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.get_char();
        }
        if self.pos >= self.chars.len() {
            return String::new();
        }

        let first = self.get_char();
        self.add_char(first);

        if first == '#' {
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_alphabetic() {
                let c = self.get_char();
                self.add_char(c);
            }
        } else {
            if !is_allowed_char(first) {
                self.lex_error(&format!("illegal character '{}'", first));
            }
            while self.pos < self.chars.len()
                && !self.chars[self.pos].is_whitespace()
                && self.chars[self.pos] != '#'
            {
                let c = self.get_char();
                if !is_allowed_char(c) {
                    self.lex_error(&format!("illegal character '{}'", c));
                }
                self.add_char(c);
            }
        }

        // checks if finished token is valid
        let lexeme = self.lexeme.clone();
        if !self.lookup(&lexeme) {
            self.lex_error(&format!("unknown annotation '{}'", lexeme));
        }

        // store annotations as uppercase
        if first == '#' {
            lexeme.to_uppercase()
        } else {
            lexeme
        }
    }
}

impl LexicalAnalyzer for LolCompiler {
    // returns next character and moves forward
    fn get_char(&mut self) -> char {
        if self.pos >= self.chars.len() {
            eprintln!("Lexical error: tried to read past the end of the file");
            process::exit(1);
        }
        let c = self.chars[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    /// adds character to current token
    fn add_char(&mut self, c: char) {
        self.lexeme.push(c);
    }

    /// checks if its a valid token
    fn lookup(&self, s: &str) -> bool {
        if s.starts_with('#') {
            ANNOTATIONS.contains(&s.to_uppercase().as_str())
        } else {
            !s.is_empty() && s.chars().all(is_allowed_char)
        }
    }
}
