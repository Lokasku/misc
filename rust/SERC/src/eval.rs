#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TType {
    LParen,
    RParen,
    Minus,
    Plus,
    Times,
    Div,
    Modulo,
    Float(f64)
}

#[derive(Debug)]
pub struct Eval {
    input: String,
    pub stack: Vec<f64>,
    pub tokens: Vec<TType>,
    start: usize,
    current: usize
}

impl Eval {
    pub fn new(input: String) -> Self {
        Self {
            input,
            stack: vec![],
            tokens: vec![],
            start: 0,
            current: 0
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.input.chars().count()
    }
    
    pub fn advance(&mut self, k: usize) -> char {
        self.current += k;
        self.input.chars().nth(self.current - 1).unwrap()
    }

    pub fn peek(&self, k: usize) -> char {
        self.input.chars().nth(self.current + k).unwrap()
    }

    pub fn number(&mut self) {
        let stop = vec![' ', ')'];
        while !self.is_at_end() && !stop.contains(&self.peek(0)) { self.advance(1); }

        let raw = self.input[self.start-1..self.current].to_string();

        if raw.chars().last().unwrap() == '.' {
            panic!("Expected a decimal, but nothing found.");
        }

        let value = raw.parse::<f64>().unwrap();

        // Is negative
        let value = if self.input.chars().nth(self.start - 2).unwrap() == '-' {
            self.tokens.pop();
            -value
        } else { value };

        self.tokens.push(TType::Float(value));
    }
    
    pub fn tokenize(&mut self) {
        while !self.is_at_end() {
            let cc = self.advance(1);

            match cc {
                ' ' => {}
                '+' => self.tokens.push(TType::Plus),
                '-' => self.tokens.push(TType::Minus),
                '*' => self.tokens.push(TType::Times),
                '/' => self.tokens.push(TType::Div),
                '%' => self.tokens.push(TType::Modulo),
                '(' => self.tokens.push(TType::LParen),
                ')' => self.tokens.push(TType::RParen),
                x => if x.is_digit(10) {
                        self.start = self.current;
                        self.number();
                    } else {
                        panic!("Unexpected token: '{}'.", x);
                    }
            }
        }
        self.start = 0; self.current = 0;
    }

    pub fn next(&mut self) -> TType {
        self.current += 1;
        *self.tokens.get(self.current - 1).unwrap()
    }

    pub fn eval_one(&mut self) -> (f64, f64) {
        self.eval();
        self.eval();
        let rm = self.stack.pop().expect("?");
        let lm = self.stack.pop().expect("??");
        (lm, rm)
    }

    pub fn eval(&mut self) {
        let ct = self.next();

        match ct {
            TType::LParen => self.eval(),
            TType::RParen => self.eval(),
            TType::Plus => {
                let res = self.eval_one();
                self.stack.push(res.0 + res.1);
            }
            TType::Minus => {
                let res = self.eval_one();
                self.stack.push(res.0 - res.1);
            }
            TType::Times => {
                let res = self.eval_one();
                self.stack.push(res.0 * res.1);
            }
            TType::Div => {
                let res = self.eval_one();
                self.stack.push(res.0 / res.1);
            }
            TType::Modulo => {
                let res = self.eval_one();
                self.stack.push(res.0 % res.1);
            }
            TType::Float(n) => self.stack.push(n),
            _ => panic!("Unexpected token.")
        }
    }
}