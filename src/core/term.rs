use crate::core::variable::Variable;
use std::fmt;
use crate::core::degree::Degree;

#[derive(Clone, PartialEq, Debug)]
pub struct Term {
    pub(crate) coefficient: f32,
    pub(crate) variable: Variable,
    pub(crate) degree: Degree,
}

impl Term {
    pub fn new(coefficient: f32, variable: Variable, degree: Degree) -> Term {
        Term { coefficient, variable, degree }
    }
    pub fn constant(constant: f32) -> Term {
        Term { coefficient: constant, variable: Variable::None, degree: Degree::Number(1.0)}
    }
    pub fn linear(coefficient: f32, variable: Variable) -> Term {
        Term {coefficient, variable, degree: Degree::Number(1.0)}
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coefficient_str = match &self.coefficient {
            x if *x == 1.0 && self.variable.to_string() != "" => "".to_string(),
            x => x.to_string(),
        };
        write!(f, "{}{}{}", coefficient_str, self.variable, self.degree)
    }
}