use crate::core::expression::Expression;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    None,
    Normal(char),
    Sine(Expression),
    Cosine(Expression),
    Tangent(Expression),
    Arcsine(Expression),
    Arccosine(Expression),
    Arctangent(Expression),
    Secant(Expression),
    Cosecant(Expression),
    Cotangent(Expression),
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let var_string = match &self {
            Variable::None => "".to_string(),
            Variable::Normal(x) => x.to_string(),
            Variable::Sine(term) => format!("sin({})", *term),
            Variable::Cosine(term) => format!("cos({})", *term),
            Variable::Tangent(term) => format!("tan({})", *term),
            Variable::Arcsine(term) => format!("arcsin({})", *term),
            Variable::Arccosine(term) => format!("arccos({})", *term),
            Variable::Arctangent(term) => format!("arctan({})", *term),
            Variable::Secant(term) => format!("sec({})", *term),
            Variable::Cosecant(term) => format!("csc({})", *term),
            Variable::Cotangent(term) => format!("cot({})", *term),
        };
        write!(f, "{}", var_string)
    }
}