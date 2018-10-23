pub fn build_scanner(code: String) {
	let scanner = Scanner {
		current: None,
		next: None,
		code,
		index: 0,
		line: 1,
		column: 1,
	};
	
	scanner.scanToken().scanToken()
}

struct Scanner {
	current: Option<Token>,
	next: Option<Token>,
	code: String,
	index: usize,
	line: u32,
	column: u32,
}

impl Scanner {
	pub fn scanToken(self) -> Self {
		self.scanTokenUtil(self.index, self.index)
	}
	
	fn scanTokenUtil(self, start: usize, current: usize) {
		if self.isAtEnd(current) {
			return Scanner {
				current: TokenType::EOF,
				next: self.current,
				index
			}
		}
	}
	
	fn isAtEnd(&self, index) {
		index >= self.code.len
	}
}

struct Token {
	t_type: TokenType,
	lexeme: String,
	line: u32,
	column: u32,
}

impl Token {
	fn new(t_type: TokenType, scanner: &Scanner) {
		Token {
			t_type,
			
		}
	}
}

enum TokenType {                                        
  // Single-character tokens.                         
  LeftParen, RightParen,                
  LeftBrace, RightBrace,                
  Comma, Dot, Minus, Plus,    
  Semicolon, Slash, Star,

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
