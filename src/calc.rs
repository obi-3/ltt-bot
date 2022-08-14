use crate::{lexer, parse};
use lexer::*;
use parse::*;

fn rec(vvec: &mut Vec<Vec<bool>>, vec: &mut Vec<bool>, n: usize, len: usize) {
    if n == 0 {
        vvec.push(vec.to_vec());
        return;
    }
    for b in [true, false] {
        vec[len - n] = b;
        rec(vvec, vec, n - 1, len);
    }
}

fn calc_prefix(op: Operator, x: Option<bool>) -> Option<bool> {
    use Operator::*;
    match x {
        Some(x) => match op {
            Not => Some(!x),
            _ => None,
        },
        None => None,
    }
}

fn calc_infix(op: Operator, x: Option<bool>, y: Option<bool>) -> Option<bool> {
    use Operator::*;
    match (x, y) {
        (Some(x), Some(y)) => match op {
            Or => Some(x | y),
            Nor => Some(!(x | y)),
            Xor => Some(x != y),
            And => Some(x & y),
            Nand => Some(!(x & y)),
            Is => Some(!x | y),
            _ => None,
        },
        (_, _) => None,
    }
}

fn calc(vec: Vec<bool>, root: Option<Box<Tree>>) -> Option<bool> {
    use Operator::*;
    use Token::*;
    match root {
        Some(root) => match root.token {
            Var(_, n) => Some(vec[n as usize]),
            True => Some(true),
            False => Some(false),
            Op(Not) => calc_prefix(Not, calc(vec, root.left)),
            Op(op @ (Or | Nor | Xor | And | Nand | Is)) => {
                calc_infix(op, calc(vec.clone(), root.left), calc(vec, root.right))
            }
            _ => None,
        },
        None => None,
    }
}

pub fn make_truth_table(str: String) -> Result<String, ParseError> {
    let lexer = Lexer::new(str.clone());
    let mut parser = Parser::new(lexer);
    let root = parser.parse()?;

    let n = parser.lexer.vnum as usize;
    let mut vec: Vec<bool> = vec![true; n];
    let mut vvec: Vec<Vec<bool>> = Vec::new();
    rec(&mut vvec, &mut vec, n, n);

    let mut truth_table = String::new();
    truth_table.push_str("```");
    truth_table.push_str(format!("input: {}\n", str).as_str());

    for var in &parser.lexer.vars {
        truth_table.push_str(format!("|{}", var).as_str());
    }
    truth_table.push_str("||f|\n");

    for v in vvec {
        for (cnt, belm) in v.clone().into_iter().enumerate() {
            let elm = if belm { 1 } else { 0 };
            truth_table.push('|');
            for _i in 1..parser.lexer.vars[cnt].len() {
                truth_table.push(' ');
            }
            truth_table.push_str(format!("{}", elm).as_str());
        }
        let val = calc(v, root.clone()).expect("Null Expression");
        let f = if val { 1 } else { 0 };
        truth_table.push_str(format!("||{}|\n", f).as_str());
    }
    truth_table.push_str("```");

    Ok(truth_table)
}
