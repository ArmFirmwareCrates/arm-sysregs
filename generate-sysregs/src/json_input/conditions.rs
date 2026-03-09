// SPDX-FileCopyrightText: Copyright The arm-sysregs Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Logic for evaluating conditions.

use arm_sysregs_json::{
    AstBinaryOp, AstBool, AstFunction, AstIdentifier, AstInteger, AstUnaryOp, Expression,
    TypesField, Value,
};
use eyre::{Context, Report, bail};
use std::{collections::BTreeMap, i64};

/// Environment for evaluating AST expressions.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Environment {
    /// Values to use for identifiers in the expression.
    pub variables: BTreeMap<String, EvalValue>,
}

/// The result of an AST expression.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EvalValue {
    Boolean(bool),
    Integer(i64),
    /// We don't have enough information to know what the value is.
    #[default]
    Unknown,
}

impl EvalValue {
    /// Returns true if this is value is either true or unknown.
    pub fn could_be_true(self) -> bool {
        matches!(self, Self::Boolean(true) | Self::Unknown)
    }
}

/// An AST expression which can be evaluated to a value.
pub trait Evaluable {
    fn eval(&self, environment: &Environment) -> Result<EvalValue, Report>;
}

impl Evaluable for Expression {
    fn eval(&self, environment: &Environment) -> Result<EvalValue, Report> {
        match self {
            Self::BinaryOp(binary_op) => binary_op.eval(environment),
            Self::Bool(boolean) => boolean.eval(environment),
            Self::Field(field) => field.eval(environment),
            Self::Function(function) => function.eval(environment),
            Self::Identifier(identifier) => identifier.eval(environment),
            Self::Integer(integer) => integer.eval(environment),
            Self::Value(value) => value.eval(environment),
            Self::UnaryOp(unary_op) => unary_op.eval(environment),
            _ => bail!("Unsupported expression {self:?}"),
        }
    }
}

impl Evaluable for AstBinaryOp {
    fn eval(&self, environment: &Environment) -> Result<EvalValue, Report> {
        let left = self.left.eval(environment)?;
        let right = self.right.eval(environment)?;
        match (left, self.op.as_str(), right) {
            (EvalValue::Boolean(left), "||", EvalValue::Boolean(right)) => {
                Ok(EvalValue::Boolean(left || right))
            }
            (EvalValue::Unknown, "||", EvalValue::Boolean(true))
            | (EvalValue::Boolean(true), "||", EvalValue::Unknown) => Ok(EvalValue::Boolean(true)),
            (EvalValue::Unknown, "||", EvalValue::Boolean(false))
            | (EvalValue::Boolean(false), "||", EvalValue::Unknown)
            | (EvalValue::Unknown, "||", EvalValue::Unknown) => Ok(EvalValue::Unknown),
            (EvalValue::Boolean(left), "&&", EvalValue::Boolean(right)) => {
                Ok(EvalValue::Boolean(left && right))
            }
            (EvalValue::Unknown, "&&", EvalValue::Boolean(false))
            | (EvalValue::Boolean(false), "&&", EvalValue::Unknown) => {
                Ok(EvalValue::Boolean(false))
            }
            (EvalValue::Unknown, "&&", EvalValue::Boolean(true))
            | (EvalValue::Boolean(true), "&&", EvalValue::Unknown)
            | (EvalValue::Unknown, "&&", EvalValue::Unknown) => Ok(EvalValue::Unknown),
            (EvalValue::Unknown, "==", _)
            | (_, "==", EvalValue::Unknown)
            | (EvalValue::Unknown, "!=", _)
            | (_, "!=", EvalValue::Unknown) => Ok(EvalValue::Unknown),
            (left, "==", right) => Ok(EvalValue::Boolean(left == right)),
            (left, "!=", right) => Ok(EvalValue::Boolean(left != right)),
            (left, op, right) => bail!("Unsupported condition operation {left:?} {op} {right:?}"),
        }
    }
}

impl Evaluable for AstUnaryOp {
    fn eval(&self, environment: &Environment) -> Result<EvalValue, Report> {
        let expr = self.expr.eval(environment)?;
        match (self.op.as_str(), expr) {
            ("!", EvalValue::Boolean(expr)) => Ok(EvalValue::Boolean(!expr)),
            ("!", EvalValue::Unknown) => Ok(EvalValue::Unknown),
            (op, expr) => bail!("Unsupported condition operation {op} {expr:?}"),
        }
    }
}

impl Evaluable for AstIdentifier {
    fn eval(&self, environment: &Environment) -> Result<EvalValue, Report> {
        if let Some(value) = environment.variables.get(&self.value) {
            Ok(*value)
        } else {
            bail!("Unexpected identifier {}", self.value)
        }
    }
}

impl Evaluable for AstFunction {
    fn eval(&self, _environment: &Environment) -> Result<EvalValue, Report> {
        match self.name.as_str() {
            "EffectiveHCR_EL2_E2H" | "HaveEL" | "ImpDefBool" | "IsFeatureImplemented" | "Text" => {
                Ok(EvalValue::Unknown)
            }
            name => bail!("Unsupported AST function {name}"),
        }
    }
}

impl Evaluable for AstInteger {
    fn eval(&self, _environment: &Environment) -> Result<EvalValue, Report> {
        Ok(EvalValue::Integer(self.value))
    }
}

impl Evaluable for AstBool {
    fn eval(&self, _environment: &Environment) -> Result<EvalValue, Report> {
        Ok(EvalValue::Boolean(self.value))
    }
}

impl Evaluable for TypesField {
    fn eval(&self, _environment: &Environment) -> Result<EvalValue, Report> {
        Ok(EvalValue::Unknown)
    }
}

impl Evaluable for Value {
    fn eval(&self, _environment: &Environment) -> Result<EvalValue, Report> {
        if self.value.starts_with('\'') && self.value.ends_with('\'') {
            Ok(EvalValue::Integer(
                i64::from_str_radix(self.value.trim_matches('\''), 2)
                    .with_context(|| format!("Parsing value {:?}", self.value))?,
            ))
        } else {
            bail!("Unexpected value format {:?}", self.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arm_sysregs_json::{AstFunction, AstIdentifier, AstInteger};

    #[test]
    fn nmi() {
        let expression = Expression::BinaryOp(AstBinaryOp {
            op: "&&".to_owned(),
            left: Box::new(Expression::Function(AstFunction {
                arguments: vec![Expression::Identifier(AstIdentifier {
                    value: "FEAT_GICv3_NMI".to_owned(),
                })],
                name: "IsFeatureImplemented".to_owned(),
                parameters: vec![],
            })),
            right: Box::new(Expression::BinaryOp(AstBinaryOp {
                op: "==".to_owned(),
                left: Box::new(Expression::Identifier(AstIdentifier {
                    value: "n".to_owned(),
                })),
                right: Box::new(Expression::Integer(AstInteger { value: 0 })),
            })),
        });

        assert_eq!(
            expression
                .eval(&Environment {
                    variables: [("n".to_owned(), EvalValue::Integer(0))]
                        .into_iter()
                        .collect(),
                })
                .unwrap(),
            EvalValue::Unknown
        );
        assert_eq!(
            expression
                .eval(&Environment {
                    variables: [("n".to_owned(), EvalValue::Integer(1))]
                        .into_iter()
                        .collect(),
                })
                .unwrap(),
            EvalValue::Boolean(false)
        );
    }
}
