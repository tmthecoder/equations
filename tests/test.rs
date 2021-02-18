#[cfg(test)]
mod tests {
    use equations::{Term, Variable};

    #[test]
    fn it_works() {
        let string = format!("{}", Term::new(1.0, Variable::None, 1.0));
        assert_eq!(string, "1");
    }

    #[test]
    fn var_test() {
        let normalVar = format!("{}", Term::new(1.0, Variable::Normal('x'), 2.0));
        assert_eq!(normalVar, "x^2");
    }

    #[test]
    fn trig_test() {
        let trig_string = format!("{}", Term::new(1.0, Variable::Sine(Box::from(Term::new(1.0, Variable::Normal('x'), 1.0))), 2.0));
        assert_eq!(trig_string, "sin(x)^2");
    }
}