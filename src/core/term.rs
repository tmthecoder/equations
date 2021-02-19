use crate::core::variable::Variable;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Term {
    pub(crate) coefficient: f32,
    pub(crate) variable: Variable,
    pub(crate) degree: f32,
}

impl Term {
    pub fn new(coefficient: f32, variable: Variable, degree: f32) -> Term {
        Term { coefficient, variable, degree }
    }
    pub fn constant(constant: f32) -> Term {
        Term { coefficient: constant, variable: Variable::None, degree: 1.0}
    }
    pub fn linear(coefficient: f32, variable: Variable) -> Term {
        Term {coefficient, variable, degree: 1.0}
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variable_str = match &self.variable {
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
        let coefficient_str = match &self.coefficient {
            x if *x == 1.0 && variable_str != "" => "".to_string(),
            x => x.to_string(),
        };
        let degree_str = match &self.degree {
            x if *x == 1.0 => "".to_string(),
            x => format!("^{}", x)
        };
        write!(f, "{}{}{}", coefficient_str, variable_str, degree_str)
    }
}