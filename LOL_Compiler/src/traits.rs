// Chose Option 2 for trait

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

