use crate::core::term::Term;
use crate::core::simplifiable::Simplifiable;
use std::ops::{AddAssign, Add, SubAssign, Sub};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    terms: Vec<Term>
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

impl AddAssign for Expression {
    fn add_assign(&mut self, other: Self) {
        let mut terms = vec![&self.terms[..], &other.terms[..]].concat();
        terms.sort_by(|a, b| b.degree.partial_cmp(&a.degree).unwrap());
        terms.simplify();
        *self = Expression::new(terms);
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = vec![&self.terms[..], &rhs.terms[..]].concat();
        terms.sort_by(|a, b| b.degree.partial_cmp(&a.degree).unwrap());
        terms.simplify();
        Expression::new(terms)
    }
}

impl SubAssign for Expression {
    fn sub_assign(&mut self, rhs: Self) {
        let mut negated = rhs.terms;
        for mut term in negated.iter_mut() {
            term.coefficient *= -1.0;
        }
        let mut terms = vec![&self.terms[..], &negated[..]].concat();
        println!("{:?}", terms);
        terms.simplify();
        *self = Expression::new(terms);
    }
}

impl Sub for Expression {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut negated = rhs.terms;
        for mut term in negated.iter_mut() {
            term.coefficient *= -1.0;
        }
        let mut terms = vec![&self.terms[..], &negated[..]].concat();
        terms.simplify();
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