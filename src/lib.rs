use std::fmt;

pub enum Variable {
    None,
    Normal(char),
    Sine(Box<Term>),
    Cosine(Box<Term>),
    Tangent(Box<Term>),
    Arcsine(Box<Term>),
    Arccosine(Box<Term>),
    Arctangent(Box<Term>),
    Secant(Box<Term>),
    Cosecant(Box<Term>),
    Cotangent(Box<Term>),
}

pub struct Term {
    coefficient: f32,
    variable: Variable,
    degree: f32,
}

impl Term {
    pub fn new(coefficient: f32, variable: Variable, degree: f32) -> Term {
        Term { coefficient, variable, degree }
    }
    // fn new_constant(coefficient: f32) -> Term {
    //     Term { coefficient, variable: Variable::NoVariable, degree: 1.0}
    // }
    // fn new_linear(coefficient: f32, variable: Variable) -> Term {
    //     Term {coefficient, variable, degree: 1.0}
    // }
    // fn new_singular(variable: Variable, degree: f32) -> Term {
    //     Term{ coefficient: 1.0, variable, degree}
    // }
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

struct Equation {
    lhs: Vec<Term>,
    rhs: Vec<Term>,
}