#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Not,
    Is,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var(String, i32),
    Op(Operator),
    True,
    False,
    Rpar,
    Lpar,
    End,
    Error,
}

type Charkind = u8;
fn match_charkind(&c: &char) -> Charkind {
    match c {
        'a'..='z' => 0,
        'A'..='Z' | '0'..='9' => 1,
        //一文字で有意なもの
        '(' | ')' | '~' | '!' => 2,
        _ => 3,
    }
}

pub fn format_string(str: String) -> Vec<String> {
    let cs: Vec<char> = str
        .split_ascii_whitespace()
        .collect::<String>()
        .chars()
        .collect();

    let mut index: usize = 0;
    let mut strs: Vec<String> = Vec::new();

    let mut prev = match_charkind(&cs[0]);
    strs.push(String::new());
    strs[index].push(cs[0]);

    for c in cs.into_iter().skip(1) {
        if prev != 2 && prev == match_charkind(&c) {
            strs[index].push(c);
        } else {
            index += 1;
            strs.push(String::new());
            strs[index].push(c);
        }
        prev = match_charkind(&c);
    }
    strs
}

#[test]
fn format_test() {
    let str = "!((X) is Y)".to_string();
    let ret = format_string(str);
    assert_eq!(ret, ["!", "(", "(", "X", ")", "is", "Y", ")"]);
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub strs: Vec<String>,
    pub position: usize,
    pub vars: Vec<String>,
    pub vnum: i32,
}

impl Lexer {
    pub fn new(str: String) -> Lexer {
        Lexer {
            strs: format_string(str),
            position: 0,
            vars: Vec::new(),
            vnum: 0,
        }
    }
    pub fn get_token(&mut self) -> Token {
        use Operator::*;
        use Token::*;

        if self.position == self.strs.len() {
            return Token::End;
        }
        let token = match self.strs[self.position].as_str() {
            "(" => Lpar,
            ")" => Rpar,
            "TRUE" | "1" => True,
            "FALSE" | "0" => False,
            "and" | "*" => Op(And),
            "nand" => Op(Nand),
            "or" | "+" => Op(Or),
            "nor" => Op(Nor),
            "xor" => Op(Xor),
            "is" | "->" => Op(Is),
            "~" | "!" => Op(Not),
            str => {
                let mut same_flag = 0;
                let mut tmp = Error;
                if let Some(same_index) = self.vars.iter().position(|x| *x == str) {
                    tmp = Var(self.vars[same_index].clone(), same_index as i32);
                    same_flag = 1;
                }
                if same_flag == 0 {
                    tmp = Var(str.to_string(), self.vnum);
                    self.vars.push(str.to_string());
                    self.vnum += 1;
                }
                tmp
            }
        };

        self.position += 1;
        token
    }
}
