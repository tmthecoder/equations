use crate::core::expression::Expression;
use crate::core::variable::Variable;

#[derive(Clone, PartialEq, Debug)]
pub enum Degree {
    Expression(Expression),
    Variable(Variable),
    Number(f32),
}