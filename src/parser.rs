use std::collections::HashMap;

use crate::compiler::LolCompiler;
use crate::traits::{Compiler, SyntaxAnalyzer};


impl LolCompiler {
    // put parser at first token
    pub fn start_parsing(&mut self) {
        self.tok_pos = 0;
        let first = self.tokens.first().cloned().unwrap_or_default();
        self.set_current_token(first);
    }

    // go to next token
    fn advance(&mut self) {
        self.tok_pos += 1;
        let next = self.tokens.get(self.tok_pos).cloned().unwrap_or_default();
        self.set_current_token(next);
    }

    // look at the token after current one
    fn peek(&self) -> String {
        self.tokens
            .get(self.tok_pos + 1)
            .map(|t| t.to_uppercase())
            .unwrap_or_default()
    }

    // check token validity
    fn check(&self, wanted: &str) -> bool {
        self.current.to_uppercase() == wanted
    }

    // checks if token is text
    fn is_text(&self) -> bool {
        !self.current.is_empty() && !self.current.starts_with('#')
    }

    // error line specifier
    fn error_line(&self) -> usize {
        self.lines
            .get(self.tok_pos)
            .or(self.lines.last())
            .copied()
            .unwrap_or(1)
    }

    // desribe current token for error messages
    fn describe_current(&self) -> String {
        if self.current.is_empty() {
            "end of file".to_string()
        } else {
            format!("'{}'", self.current)
        }
    }

    // check expected token and if its correct move on, wrong throw an error
    fn expect(&mut self, wanted: &str) -> Result<(), String> {
        if self.check(wanted) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Syntax error at line {}: expected '{}' but found {}",
                self.error_line(),
                wanted,
                self.describe_current()
            ))
        }
    }

    fn emit(&mut self, s: &str) {
        self.html.push_str(s);
    }

    // checks if innter text is in a valid place
    fn starts_inner_text(&self, allow_list: bool) -> bool {
        if self.is_text() || self.check("#OBTW") || self.check("#NEWLINE") {
            return true;
        }
        if allow_list && self.check("#MAEK") && self.peek() == "LIST" {
            return true;
        }
        if self.check("#GIMMEH") {
            let next = self.peek();
            return next == "BOLD" || next == "LINX" || next == "VAR";
        }
        false
    }

    // look for variable, static scoping
    fn find_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
}



impl SyntaxAnalyzer for LolCompiler {
    // <lolcode> ::= HAI <comments> <head> <optional-variable-define> <body> KBYE
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.expect("#HAI")?;
        self.emit("<html>\n");
        self.scopes.push(HashMap::new()); // the global scope

        while self.check("#OBTW") {
            self.parse_comment()?;
        }
        self.parse_head()?;
        if self.check("#IHAZ") {
            self.parse_variable_define()?;
        }
        self.parse_body()?;

        self.expect("#KBYE")?;
        self.scopes.pop();
        self.emit("</html>\n");

        // nothing allowed after #KBYE
        if self.tok_pos < self.tokens.len() {
            return Err(format!(
                "Syntax error at line {}: found {} after #KBYE",
                self.error_line(),
                self.describe_current()
            ));
        }
        Ok(())
    }

    // <head> ::= MAEK HEAD <title> MKAY | empty
    fn parse_head(&mut self) -> Result<(), String> {
        if self.check("#MAEK") && self.peek() == "HEAD" {
            self.expect("#MAEK")?;
            self.expect("HEAD")?;
            self.emit("<head>\n");
            self.parse_title()?;
            self.expect("#MKAY")?;
            self.emit("</head>\n");
        }
        Ok(())
    }

    // <title> ::= GIMMEH TITLE TEXT OIC
    fn parse_title(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("TITLE")?;
        let mut words: Vec<String> = Vec::new();
        while self.is_text() {
            words.push(self.current_token());
            self.advance();
        }
        self.expect("#OIC")?;
        self.emit(&format!("<title> {} </title>\n", words.join(" ")));
        Ok(())
    }

    // <comment> ::= OBTW TEXT TLDR
    fn parse_comment(&mut self) -> Result<(), String> {
        self.expect("#OBTW")?;
        let mut words: Vec<String> = Vec::new();
        while self.is_text() {
            words.push(self.current_token());
            self.advance();
        }
        self.expect("#TLDR")?;
        self.emit(&format!("<!-- {} -->\n", words.join(" ")));
        Ok(())
    }

    // <body> ::= <inner-body> <body> | empty
    fn parse_body(&mut self) -> Result<(), String> {
        loop {
            if self.check("#MAEK") && self.peek() == "PARAGRAF" {
                self.parse_paragraph()?;
            } else if self.starts_inner_text(true) {
                self.parse_inner_text()?;
            } else {
                break;
            }
        }
        Ok(())
    }

    // <paragraph> ::= MAEK PARAGRAF <optional-variable-define> <inner-paragraph> MKAY
    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.expect("#MAEK")?;
        self.expect("PARAGRAF")?;
        self.emit("<p> ");
        self.scopes.push(HashMap::new());     // a paragraph is a new block, so a new scope
        if self.check("#IHAZ") {
            self.parse_variable_define()?;
        }
        self.parse_inner_paragraph()?;
        self.expect("#MKAY")?;
        self.scopes.pop();                   // leaving the block ends its scope
        self.emit("</p>\n");
        Ok(())
    }

    // <inner-paragraph> ::= <inner-text> <inner-paragraph> | empty
    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        while self.starts_inner_text(true) {
            self.parse_inner_text()?;
        }
        if self.check("#MAEK") && self.peek() == "PARAGRAF" {
            return Err(format!(
                "Syntax error at line {}: a paragraph cannot be inside another paragraph",
                self.error_line()
            ));
        }
        Ok(())
    }

    // <inner-text> ::= <variable-use> | <comment> | <bold> | <list> | <link> | <newline> | TEXT
    // check current token and choose rule based on that
    fn parse_inner_text(&mut self) -> Result<(), String> {
        if self.is_text() {
            self.parse_text()
        } else if self.check("#OBTW") {
            self.parse_comment()
        } else if self.check("#NEWLINE") {
            self.parse_newline()
        } else if self.check("#MAEK") && self.peek() == "LIST" {
            self.parse_list()
        } else if self.check("#GIMMEH") {
            let next = self.peek();
            if next == "BOLD" {
                self.parse_bold()
            } else if next == "LINX" {
                self.parse_link()
            } else if next == "VAR" {
                self.parse_variable_use()
            } else {
                Err(format!(
                    "Syntax error at line {}: '#GIMMEH {}' is not allowed here",
                    self.error_line(),
                    next
                ))
            }
        } else {
            Err(format!(
                "Syntax error at line {}: unexpected {}",
                self.error_line(),
                self.describe_current()
            ))
        }
    }

    // <variable-define> ::= IHAZ VARNAME ITIZ VARVALUE MKAY
    // saves the variable in the innermost scope.
    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.expect("#IHAZ")?;

        // the name must be one word made only of letters
        if !self.is_text() || !self.current.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(format!(
                "Syntax error at line {}: variable name must be a single word of letters, found {}",
                self.error_line(),
                self.describe_current()
            ));
        }
        let name = self.current_token();
        self.advance();

        self.expect("#ITIZ")?;

        // the value is one or more words
        let mut words: Vec<String> = Vec::new();
        while self.is_text() {
            words.push(self.current_token());
            self.advance();
        }
        if words.is_empty() {
            return Err(format!(
                "Syntax error at line {}: variable '{}' needs a value after #ITIZ",
                self.error_line(),
                name
            ));
        }
        self.expect("#MKAY")?;

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, words.join(" "));
        }
        Ok(())
    }

    // <variable-use> ::= GIMMEH VAR VARNAME OIC
    // makes sure variable is already defined somewhere 
    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("VAR")?;

        if !self.is_text() {
            return Err(format!(
                "Syntax error at line {}: expected a variable name but found {}",
                self.error_line(),
                self.describe_current()
            ));
        }
        let name = self.current_token();
        let line = self.error_line();

        let value = match self.find_variable(&name) {
            Some(v) => v,
            None => {
                return Err(format!(
                    "Static semantic error at line {}: variable '{}' used before it was defined",
                    line, name
                ));
            }
        };
        self.advance();
        self.expect("#OIC")?;
        self.emit(&format!("{} ", value));
        Ok(())
    }

    // <bold> ::= GIMMEH BOLD TEXT OIC
    fn parse_bold(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("BOLD")?;
        self.emit("<b> ");
        while self.is_text() {
            self.parse_text()?;
        }
        self.expect("#OIC")?;
        self.emit("</b> ");
        Ok(())
    }

    // <list> ::= MAEK LIST <list-item> <list-items> MKAY
    fn parse_list(&mut self) -> Result<(), String> {
        self.expect("#MAEK")?;
        self.expect("LIST")?;
        self.emit("<ul>\n");
        self.parse_list_items()?;
        self.expect("#MKAY")?;
        self.emit("</ul>\n");
        Ok(())
    }

    // <list-item> ::= GIMMEH ITEM <optional-variable-define> <inner-list> OIC
    fn parse_list_items(&mut self) -> Result<(), String> {
        if !(self.check("#GIMMEH") && self.peek() == "ITEM") {
            return Err(format!(
                "Syntax error at line {}: a list must start with #GIMMEH ITEM, found {}",
                self.error_line(),
                self.describe_current()
            ));
        }
        while self.check("#GIMMEH") && self.peek() == "ITEM" {
            self.expect("#GIMMEH")?;
            self.expect("ITEM")?;
            self.emit("<li> ");
            self.scopes.push(HashMap::new());       // each item is its own block
            if self.check("#IHAZ") {
                self.parse_variable_define()?;
            }
            self.parse_inner_list()?;
            self.expect("#OIC")?;
            self.scopes.pop();
            self.emit("</li>\n");
        }
        Ok(())
    }

    // <inner-list> ::= <list-content> <inner-list> | empty   (no nested lists)
    fn parse_inner_list(&mut self) -> Result<(), String> {
        while self.starts_inner_text(false) {
            self.parse_inner_text()?;
        }
        Ok(())
    }

    // <link> ::= GIMMEH LINX ADDRESS OIC
    fn parse_link(&mut self) -> Result<(), String> {
        self.expect("#GIMMEH")?;
        self.expect("LINX")?;
        if !self.is_text() {
            return Err(format!(
                "Syntax error at line {}: expected an address after LINX but found {}",
                self.error_line(),
                self.describe_current()
            ));
        }
        let address = self.current_token();
        self.advance();
        self.expect("#OIC")?;
        self.emit(&format!("<a href=\"{}\">{}</a> ", address, address));
        Ok(())
    }

    // <newline> ::= NEWLINE
    fn parse_newline(&mut self) -> Result<(), String> {
        self.expect("#NEWLINE")?;
        self.emit("<br>\n");
        Ok(())
    }

    // TEXT: one plain word
    fn parse_text(&mut self) -> Result<(), String> {
        if !self.is_text() {
            return Err(format!(
                "Syntax error at line {}: expected text but found {}",
                self.error_line(),
                self.describe_current()
            ));
        }
        let word = self.current_token();
        self.emit(&format!("{} ", word));
        self.advance();
        Ok(())
    }
}
