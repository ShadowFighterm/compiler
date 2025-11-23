#[derive(Clone, Debug)]
pub struct Quad {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

impl Quad {
    pub fn new<S: Into<String>>(op: S, arg1: S, arg2: S, result: S) -> Self {
        Self {
            op: op.into(),
            arg1: arg1.into(),
            arg2: arg2.into(),
            result: result.into(),
        }
    }

    pub fn to_string(&self) -> String {
        match self.op.as_str() {
            "copy" => format!("{} = {}", self.result, self.arg1),
            "goto" => format!("goto {}", self.result),
            "if_false" => format!("if_false {} goto {}", self.arg1, self.result),
            "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" =>
                format!("{} = {} {} {}", self.result, self.arg1, self.op, self.arg2),
            "neg" | "not" => format!("{} = {} {}", self.result, self.op, self.arg1),
            "return" => format!("return {}", self.arg1),
            "label" => format!("{}:", self.result),
            _ =>
                format!("/* Unhandled Quad: {} {} {} {} */", self.op, self.arg1, self.arg2, self.result),
        }
    }
}
















