#[derive(Debug, Clone, PartialEq)]
pub enum ParseResult {
    Complete(Vec<String>),
    NeedsContinuation(String),
}

pub struct ShellParser {
    input: Vec<char>,
    pos: usize,
    current_word: String,
    words: Vec<String>,
    in_squote: bool,
    in_dquote: bool,
    escaped: bool,
}

impl ShellParser {
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            pos: 0,
            current_word: String::new(),
            words: Vec::new(),
            in_squote: false,
            in_dquote: false,
            escaped: false,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<ParseResult, ()> {
        self.input = input.chars().collect();
        self.pos = 0;
        self.current_word.clear();
        self.words.clear();
        self.in_squote = false;
        self.in_dquote = false;
        self.escaped = false;

        while self.pos < self.input.len() {
            let ch = self.input[self.pos];

            if self.escaped {
                self.handle_escaped_char(ch);
                self.escaped = false;
                self.pos += 1;
                continue;
            }

            match ch {
                '\\' => self.handle_backslash(),
                '\'' => self.handle_single_quote(),
                '"' => self.handle_double_quote(),
                ' ' | '\t' => self.handle_whitespace(),
                '#' => {
                    if !self.in_squote && !self.in_dquote {
                        break;
                    }
                    self.current_word.push(ch);
                }
                _ => self.current_word.push(ch),
            }

            self.pos += 1;
        }

        if self.in_squote {
            return Ok(ParseResult::NeedsContinuation("squote> ".to_string()));
        }
        if self.in_dquote {
            return Ok(ParseResult::NeedsContinuation("dquote> ".to_string()));
        }
        if self.escaped {
            return Ok(ParseResult::NeedsContinuation("> ".to_string()));
        }

        if !self.current_word.is_empty() {
            self.words.push(self.current_word.clone());
        }

        Ok(ParseResult::Complete(self.words.clone()))
    }

    fn handle_backslash(&mut self) {
        if self.in_squote {
            self.current_word.push('\\');
        } else if self.pos + 1 >= self.input.len() {
            self.escaped = true;
        } else {
            let next_char = self.input[self.pos + 1];
            if next_char == '\n' {
                self.current_word.push('\n');
                self.pos += 1;
            } else if self.in_dquote {
                match next_char {
                    '"' | '\\' | '$' | '`' => {
                        self.current_word.push(next_char);
                        self.pos += 1;
                    }
                    'n' => {
                        if !self.words.is_empty() && self.words[0] == "echo" {
                            self.current_word.push('\n');
                        }
                        self.current_word.push_str("\\n");
                        self.pos += 1;
                    }
                    _ => {
                        self.current_word.push('\\');
                        self.current_word.push(next_char);
                        self.pos += 1;
                    }
                }
            } else {
                self.current_word.push(next_char);
                self.pos += 1;
            }
        }
    }

    fn handle_escaped_char(&mut self, ch: char) {
        if ch == '\n' {
            return;
        }
        self.current_word.push(ch);
    }

    fn handle_single_quote(&mut self) {
        if self.in_dquote {
            self.current_word.push('\'');
        } else {
            self.in_squote = !self.in_squote;
        }
    }

    fn handle_double_quote(&mut self) {
        if self.in_squote {
            self.current_word.push('"');
        } else {
            self.in_dquote = !self.in_dquote;
        }
    }

    fn handle_whitespace(&mut self) {
        if self.in_squote || self.in_dquote {
            self.current_word.push(self.input[self.pos]);
        } else {
            if !self.current_word.is_empty() {
                self.words.push(self.current_word.clone());
                self.current_word.clear();
            }
        }
    }
}

pub fn parse_command(input: &str) -> Result<ParseResult, ()> {
    let mut parser = ShellParser::new();
    parser.parse(input)
}
