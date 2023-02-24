#[derive(Debug)]
enum Token {
    Plus,
    Minus,
    Right,
    Left,
    BeginLoop,
    EndLoop,
    Input, 
    Output,
}

#[derive(Debug)]
struct Parser {
    index: usize,
    source: String,
}

impl Parser {
    pub fn new(source: String)-> Self{
       Self { 
            index: 0,
            source: source.replace(" ", "").replace("\n", ""),
        }
    }
    fn parse(&mut self) -> Vec<Token>{
        let mut parsed: Vec<Token> = Vec::new();
        while self.has_next() {
            match self.source.chars().nth(self.index).unwrap() {
                '+' => parsed.push(Token::Plus),
                '-' => parsed.push(Token::Minus),
                '>' => parsed.push(Token::Right),
                '<' => parsed.push(Token::Left),
                '[' => parsed.push(Token::BeginLoop),
                ']' => parsed.push(Token::EndLoop),
                ',' => parsed.push(Token::Input),
                '.' => parsed.push(Token::Output),
                n => panic!("Invalid input {} at position {}", n, self.index)
            }
            self.next()
        }
        parsed
    }
    fn has_next(&self) -> bool {
        match self.source.chars().skip(self.index).next() {
            None => false,
            Some(_) => true,
        }
    }
    fn next(&mut self) {
        self.index += 1; 
    }
}

struct Runtime {
    memory: Vec<i32>,
    parsed: Vec<Token>,
    index: usize,
    loop_index: Vec<usize>,
    loop_depth: usize,
    mem_ptr: usize,

}
impl Runtime {
    fn run(&mut self){
       while self.has_next() {
            while self.memory.len() <= self.mem_ptr {
                self.memory.push(0);
            }
            match self.parsed.get(self.index).unwrap() {
                Token::Plus => self.memory[self.mem_ptr] += 1,
                Token::Minus => self.memory[self.mem_ptr] -= 1,
                Token::Right => self.mem_ptr += 1,
                Token::Left => self.mem_ptr -= 1,
                Token::BeginLoop => {
                    self.loop_index.push(self.index);
                    self.loop_depth += 1;
                },
                Token::EndLoop => {
                    if self.memory[self.mem_ptr] == 0 {
                        self.loop_depth -= 1;
                        self.loop_index.pop();
                    } else {
                        //eprintln!("returning to {} from {}", self.loop_index[self.loop_depth - 1], self.index);
                        self.index = self.loop_index[self.loop_depth - 1]
                    }
                },
                Token::Input => todo!(),
                Token::Output => print!("{} ", self.memory[self.mem_ptr]),
                _ => unreachable!(),
            };
            //eprintln!("memory {:?}", self.memory);
            self.index += 1;
        }
    }
    fn has_next(&self) -> bool{
        if self.parsed.len() <= self.index {
            false
        } else {
            true
        }
    }

}


fn main() {
    let mut parser = Parser::new(String::from(">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]
>++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++
.------.--------.[-]>++++++++[<++++>- ]<+.[-]++++++++++."));
    let mut runtime = Runtime {
        memory : Vec::new(),
        parsed : parser.parse(),
        index: 0,
        loop_index: Vec::new(),
        loop_depth: 0,
        mem_ptr: 0,
    };
    eprintln!("{:?}", runtime.parsed);
    runtime.run();
}
