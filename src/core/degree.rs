use crate::core::expression::Expression;
use crate::core::variable::Variable;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, PartialEq, Debug)]
pub enum Degree {
    Expression(Expression),
    Variable(Variable),
    Number(f32),
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let deg_string = match &self {
            Degree::Number(x) if x == 1.0 => "".to_string(),
            Degree::Number(x) => format!("^{}", x.to_string()),
            Degree::Variable(x) => format!("^{}", x),
            Degree::Expression(x) => format!("^{}", x)
        };
        write!(f, "{}", deg_string)
    }
}