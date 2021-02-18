use std::fmt;
use std::ops::Add;

#[derive(Clone, PartialEq,)]
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

#[derive(Clone, PartialEq,)]
pub struct Term {
    coefficient: f32,
    variable: Variable,
    degree: f32,
}

#[derive(Clone, PartialEq,)]
pub struct Expression {
    terms: Vec<Term>
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

impl Expression {
    pub fn new(terms: Vec<Term>) -> Expression {
        Expression { terms }
    }

    pub fn from_single(term: Term) -> Expression {
        Expression { terms: vec![term] }
    }
}

impl From<Term> for Expression {
    fn from(term: Term) -> Self {
        Expression::from_single(term)
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = [&self.terms[..], &rhs.terms[..]].concat();
        terms.sort_by(|a, b| b.degree.partial_cmp(&a.degree).unwrap());
        println!("{}", terms.len());
        let mut i = 0;
        while i < terms.len()-1 {
            if terms[i].degree == terms[i+1].degree && terms[i].variable == terms[i+1].variable {
                let term = &terms[i];
                let term2 = &terms[i+1];
                terms[i] = Term::new(term.coefficient + term2.coefficient, term.variable.clone(), term.degree);
                terms.remove(i+1);
                continue;
            }
            i+=1;
        }
        Expression::new(terms)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, term) in self.terms.iter().enumerate() {
            if term.coefficient > 0.0 && index != 0 {
                write!(f, "+")?;
            }
            write!(f, "{}", term)?;
        }
        Result::Ok(())
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

struct Equation {
    lhs: Vec<Term>,
    rhs: Vec<Term>,
}