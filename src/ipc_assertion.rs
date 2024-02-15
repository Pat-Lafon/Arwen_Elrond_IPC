use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum AssertionType {
    Bool,
    Int,
    Generic(String),
}

impl Display for AssertionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssertionType::Bool => write!(f, "bool"),
            AssertionType::Int => write!(f, "int"),
            AssertionType::Generic(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Assertion {
    True,
    Predicate(Pred, Vec<String>),
    Op(AssertionOp, String, String),
    Implies(Box<Assertion>, Box<Assertion>),
    Not(Box<Assertion>),
    And(Vec<Assertion>),
    Or(Vec<Assertion>),
    Iff(Box<Assertion>, Box<Assertion>),
}

/* impl Assertion {
    pub fn size(&self) -> u32 {
        match self {
            Assertion::True => 1,
            Assertion::Iff(p1, p2) | Assertion::Implies(p1, p2) => 1 + p1.size() + p2.size(),
            Assertion::Ite(p1, p2, p3) => 1 + p1.size() + p2.size() + p3.size(),
            Assertion::Not(p) => 1 + p.size(),
            Assertion::And(p_vec) | Assertion::Or(p_vec) => {
                1 + p_vec.iter().map(Assertion::size).sum::<u32>()
            }
        }
    }
} */

impl Display for Assertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assertion::Implies(lhs, rhs) => write!(f, "implies ({lhs}) ({rhs})"),
            Assertion::Not(pred) => write!(f, "!{pred}"),
            Assertion::And(preds) => {
                write!(f, "{}", preds.iter().map(|p| format!("{}", p)).join(" && "))
            }
            Assertion::Or(preds) => {
                write!(f, "{}", preds.iter().map(|p| format!("{}", p)).join(" || "))
            }
            Assertion::Iff(lhs, rhs) => write!(f, "iff ({lhs}) ({rhs})"),
            Assertion::Predicate(p1, v) => write!(
                f,
                "{p1} {v}",
                p1 = p1,
                v = v.iter().map(ToString::to_string).join(" ")
            ),
            Assertion::Op(o, v1, v2) => write!(f, "{v1} {o} {v2}",),
            Assertion::True => write!(f, "true"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AssertionOp {
    Eq,
    Le,
    Ge,
    Ne,
    Lt,
    Gt,
}

impl Display for AssertionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssertionOp::Eq => write!(f, "="),
            AssertionOp::Le => write!(f, "<="),
            AssertionOp::Ge => write!(f, ">="),
            AssertionOp::Ne => write!(f, "!="),
            AssertionOp::Lt => write!(f, "<"),
            AssertionOp::Gt => write!(f, ">"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FreeVar(pub AssertionType, pub String);

impl Display for FreeVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FreeVar(ty, name) = self;
        write!(f, "{name} : {ty}")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tpedvar(pub AssertionType, pub String);

impl Display for Tpedvar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Tpedvar(ty, name) = self;
        write!(f, "({name} : {ty})")
    }
}

/* #[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ForallFormula(pub Vec<FreeVar>, pub Assertion);

impl Display for ForallFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ForallFormula(vars, formula) = self;
        if vars.is_empty() {
            write!(f, "{formula}")
        } else {
            write!(
                f,
                "forall {} . {formula}",
                (vars.iter().map(ToString::to_string).join(","))
            )
        }
    }
} */

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Spec(pub Vec<Tpedvar>, pub Assertion);

impl Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Spec(vars, formula) = self;
        if vars.is_empty() {
            write!(f, "{formula}")
        } else {
            write!(
                f,
                "{} ⊢ {formula}",
                (vars.iter().map(ToString::to_string).join(" "))
            )
        }
    }
}

/// See translate/translate.ml -> TenvEngine -> known_preds
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Pred {
    length,
    sorted,
    member,
    head,
    order,
    once,
    left,
    right,
    para,
    ance,
    root,
}

impl Display for Pred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pred::length => write!(f, "len"),
            Pred::member => write!(f, "mem"),
            Pred::head => write!(f, "hd"),
            Pred::order => write!(f, "ord"),
            Pred::left => write!(f, "left"),
            Pred::right => write!(f, "right"),
            Pred::sorted => write!(f, "sorted"),
            Pred::once => write!(f, "once"),
            Pred::para => write!(f, "para"),
            Pred::ance => write!(f, "ance"),
            Pred::root => write!(f, "root"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AssertionPredicates(pub Vec<Pred>);

impl Display for AssertionPredicates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "let preds = [| {} |]",
            self.0.iter().map(ToString::to_string).join("; ")
        )
    }
}

#[derive(Debug)]
pub struct AssertionFile {
    pub preds: AssertionPredicates,
    pub pre_spec: Option<Spec>,
    pub post_spec: Spec,
}

impl Display for AssertionFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AssertionFile {
            preds,
            pre_spec,
            post_spec,
        } = self;
        writeln!(f, "{preds}",)?;
        if let Some(pre_spec) = pre_spec {
            pre_spec.spec_writer_for_file(f, SpecKind::Pre)?;
        }
        writeln!(f)?;

        post_spec.spec_writer_for_file(f, SpecKind::Post)?;

        Ok(())
    }
}

enum SpecKind {
    Pre,
    Post,
}

impl Display for SpecKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecKind::Pre => write!(f, "pre"),
            SpecKind::Post => write!(f, "post"),
        }
    }
}

impl Spec {
    fn spec_writer_for_file(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        kind: SpecKind,
    ) -> std::fmt::Result {
        let Spec(vars, formula) = self;
        write!(
            f,
            "let {kind} {} =\n  {}\n",
            vars.iter().map(ToString::to_string).join(" "),
            formula
        )
    }
}
