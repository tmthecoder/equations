use crate::core::term::Term;

pub trait Simplifiable {
    fn simplify(&mut self);
}

impl Simplifiable for Vec<Term> {
    fn simplify(&mut self) {
        for i in 0..self.len()-1 {
            let mut j = i+1;
            while j < self.len() {
                if self[i].degree == self[j].degree && self[i].variable == self[j].variable {
                    let term = &self[i];
                    let term2 = &self[j];
                    self[i] = Term::new(term.coefficient + term2.coefficient, term.variable.clone(), term.degree);
                    self.remove(j);
                    continue;
                }
                j+=1;
            }
        }
    }
}