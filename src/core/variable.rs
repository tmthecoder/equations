use crate::core::expression::Expression;

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