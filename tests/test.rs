#[cfg(test)]
mod tests {
    use equations::{Term, Variable, Expression};

    #[test]
    fn it_works() {
        let string = format!("{}", Term::new(1.0, Variable::None, 1.0));
        assert_eq!(string, "1");
    }

    #[test]
    fn var_test() {
        let normal_var = format!("{}", Term::new(1.0, Variable::Normal('x'), 2.0));
        assert_eq!(normal_var, "x^2");
    }

    #[test]
    fn trig_test() {
        let t1 = Term::new(1.0, Variable::Normal('x'), 1.0);
        let t2 = Term::new(1.0, Variable::Normal('x'), 1.0);
        let t3 = Term::new(1.0, Variable::Normal('x'), 2.0);
        let t4 = Term::new(1.0, Variable::Normal('x'), 2.0);
        let t5 = Term::new(1.0, Variable::Cosine(Term::new(1.0, Variable::Normal('x'), 1.0).into()), 2.0);
        let t6 = Term::new(1.0, Variable::Cosine(Term::new(1.0, Variable::Normal('x'), 4.0).into()), 1.0);
        let jointEx = Expression::new(vec![t1, t2, t3, t4, t5, t6]);
        let ex1: Expression = Term::new(1.0, Variable::Normal('x'), 1.0).into();
        let ex2: Expression = Term::new(2.0, Variable::Normal('x'), 1.0).into();
        let ex3: Expression = Term::new(5.0, Variable::Normal('x'), 1.0).into();
        let ex4: Expression = Term::new(5.0, Variable::Normal('x'), 2.0).into();
        let ex5: Expression = Term::new(5.0, Variable::Cosine(Term::new(1.0, Variable::Normal('x'), 1.0).into()), 2.0).into();
        let ex6: Expression = Term::new(5.0, Variable::Arcsine(Term::new(1.0, Variable::Normal('x'), 1.0).into()), 2.0).into();
        let ex7: Expression = Term::new(5.0, Variable::Sine(Term::new(1.0, Variable::Normal('x'), 1.0).into()), 2.0).into();

        let added = ex1 + ex2 + ex3 + ex4 + ex5 + ex6 + ex7 + jointEx;
        let trig_string = format!("{}", Term::new(1.0, Variable::Sine(Term::new(2.0, Variable::Normal('x'), 1.0).into()), 2.0));
        println!("{}", added);
        assert_eq!(trig_string, "sin(2x)^2");
    }
}
