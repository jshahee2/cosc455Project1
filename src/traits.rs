// Chose Option 2 for syntax trait

pub trait SyntaxAnalyzer {
  fn parse_lolcode(&mut self) -> Result<(), String>;
  fn parse_head(&mut self) -> Result<(), String>;
  fn parse_title(&mut self) -> Result<(), String>;
  fn parse_comment(&mut self) -> Result<(), String>;
  fn parse_body(&mut self) -> Result<(), String>;
  fn parse_paragraph(&mut self) -> Result<(), String>;
  fn parse_inner_paragraph(&mut self) -> Result<(), String>;
  fn parse_inner_text(&mut self) -> Result<(), String>;
  fn parse_variable_define(&mut self) -> Result<(), String>;
  fn parse_variable_use(&mut self) -> Result<(), String>;
  fn parse_bold(&mut self) -> Result<(), String>;
  fn parse_list(&mut self) -> Result<(), String>;
  fn parse_list_items(&mut self) -> Result<(), String>;
  fn parse_inner_list(&mut self) -> Result<(), String>;
  fn parse_link(&mut self) -> Result<(), String>;
  fn parse_newline(&mut self) -> Result<(), String>;
  fn parse_text(&mut self) -> Result<(), String>;
}

pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}
