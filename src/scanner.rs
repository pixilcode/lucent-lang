pub fn build_scanner(code: String) -> Scanner {
    let scanner = Scanner {
		current: Token::error(String::from("No available token"), 1, 1), // The tokens have not yet been scanned
		next: Token::error(String::from("No available token"), 1, 1),
		code,
		index: 0,
		line: 1,
		column: 1,
	};
	
	scanner.scan_token().scan_token()
}

pub struct Scanner {
	current: Token,
	next: Token,
	code: String,
	index: usize,
	line: u32,
	column: u32,
}

impl Scanner {
	
	pub fn current_token(&self) -> Token {
		self.current.clone()
	}
	
	fn get_char(&self, index: usize) -> &str {
		&self.code[index..=index]
	}
	
	fn match_char(&self, character: &str, index: usize) -> bool {
		!self.is_at_end(index) && self.get_char(index) == character
	}
	
	pub fn scan_token(self) -> Self {
		let scanner = self.skip_whitespace();
		let index = scanner.index;
		scanner.scan_token_util(index, index)
	}
	
	fn scan_token_util(self, start: usize, current: usize) -> Self {
		if self.is_at_end(current) {
			return Scanner {
				current: self.next,
				next: Token::new(TokenType::EOF, String::from(&self.code[start..current]), self.line, self.column),
				index: current,
				..self
			};
		}
		
		
		
		match self.get_char(current) {
			"(" => self.add_token(TokenType::LeftParen, start, current),
			")" => self.add_token(TokenType::RightParen, start, current),
			"{" => self.add_token(TokenType::LeftBrace, start, current),
			"}" => self.add_token(TokenType::RightBrace, start, current),
			";" => self.add_token(TokenType::Semicolon, start, current),
			"," => self.add_token(TokenType::Comma, start, current),
			"." => self.add_token(TokenType::Dot, start, current),
			"-" => self.add_token(TokenType::Minus, start, current),
			"+" => self.add_token(TokenType::Plus, start, current),
			"/" => self.add_token(TokenType::Slash, start, current),
			"*" => self.add_token(TokenType::Star, start, current),
			"!" => {
				if self.match_char("=", current+1) {
					self.add_token(TokenType::BangEqual, start, current+1)
				} else {
					self.add_token(TokenType::Bang, start, current)
				}
			},
			"=" => {
				if self.match_char("=", current+1) {
					self.add_token(TokenType::EqualEqual, start, current+1)
				} else {
					self.add_token(TokenType::Equal, start, current)
				}
			},
			"<" => {
				if self.match_char("=", current+1) {
					self.add_token(TokenType::LessEqual, start, current+1)
				} else {
					self.add_token(TokenType::Less, start, current)
				}
			},
			">" => {
				if self.match_char("=", current+1) {
					self.add_token(TokenType::GreaterEqual, start, current+1)
				} else {
					self.add_token(TokenType::Greater, start, current)
				}
			}
			_ => self.unexpected_char(current),
		}
	}
	
	fn skip_whitespace(self) -> Self {
		let index = self.index;
		self.skip_ws_util(index)
	}
	
	fn skip_ws_util(self, index: usize) -> Self {
		if self.is_at_end(index) {
			Scanner {
				index,
				..self
			}
		} else {
			self
		}
	}
	
	fn add_token(self, t_type: TokenType, start: usize, current: usize) -> Self {
		Scanner {
			current: self.next,
			next: Token::new(t_type, String::from(&self.code[start..=current]), self.line, self.column),
			index: current + 1,
			column: self.column + (current - start) as u32 + 1,
			..self
		}
	}
	
	fn unexpected_char(self, index: usize) -> Self {
		Scanner {
			current: self.next,
			next: Token::error(String::from("Unexpected character"), self.line, self.column),
			index: index + 1,
			column: self.column + 1,
			..self
		}
	}
	
	fn is_at_end(&self, index: usize) -> bool {
		index >= self.code.len()
	}
}

#[derive(Debug, Clone)]
pub struct Token {
	t_type: TokenType,
	lexeme: String,
	line: u32,
	column: u32,
}

impl Token {
	fn new(t_type: TokenType, lexeme: String, line: u32, column: u32) -> Self {
		Token {
			t_type,
			lexeme,
			line,
			column,
		}
	}
	
	fn error(message: String, line: u32, column: u32) -> Self {
		Token {
			t_type: TokenType::Error,
			lexeme: message,
			line,
			column,
		}
	}
	
	pub fn token_type(&self) -> TokenType {
		self.t_type.clone()
	}
	
	pub fn lexeme(&self) -> String {
		self.lexeme.clone()
	}
	
	pub fn line(&self) -> u32 {
		self.line
	}
	
	pub fn column(&self) -> u32 {
		self.column
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {                                        
  // Single-character tokens.                         
  LeftParen, RightParen,                
  LeftBrace, RightBrace,                
  Comma, Dot, Minus, Plus,    
  Semicolon, Slash, Star,
  Question,

  // One or two character tokens.                     
  Bang, BangEqual,                       
  Equal, EqualEqual,                     
  Greater, GreaterEqual,                 
  Less, LessEqual,                       

  // Literals.                                        
  Identifier, String, Number,       

  // Keywords.                                        
  And, Class, Else, False,    
  Fun, For, If, Nil, Or,
  Print, Return, Super, This, 
  True, Var, While,                 

  Error,                                        
  EOF,
}
