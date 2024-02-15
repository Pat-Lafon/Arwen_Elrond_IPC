use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::ipc_assertion::AssertionType;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArwenSetup {
    pub sourcefile: String,
    pub assertionfile: String,
    pub outputdir: String,
    pub client_name: String,
    pub predicates: ElrondPredicates,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArwenMessage {
    Start,
    Test,
    Setup(ArwenSetup),
    Message(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElrondMessage {
    Message(String),
    Result(ElrondResult),
}

impl Display for ElrondMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondMessage::Message(msg) => write!(f, "Message: {}", msg),
            ElrondMessage::Result(result) => write!(f, "Result:\n{}", result),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElrondValue {
    L(Vec<i64>),
    I(i64),
    B(bool),
    NotADt,
}

impl Display for ElrondValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondValue::L(l) => write!(f, "[{}]", l.iter().join(", ")),
            ElrondValue::I(i) => write!(f, "{}", i),
            ElrondValue::B(b) => write!(f, "{}", b),
            ElrondValue::NotADt => write!(f, "NotADt"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElrondLiteral {
    Int(i64),
    Bool(bool),
    IntList(Vec<i64>),
}

impl Display for ElrondLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondLiteral::Int(i) => write!(f, "{i}"),
            ElrondLiteral::Bool(b) => write!(f, "{b}"),
            ElrondLiteral::IntList(l) => write!(f, "[{}]", l.iter().join(", ")),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ElrondType {
    Bool,
    Int,
    IntList,
    IntTree,
    IntTreeI,
    IntTreeB,
    Tuple(Vec<ElrondType>),
    Generic(String),
    Arrow(Box<ElrondType>, Box<ElrondType>),
}

impl Display for ElrondType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondType::Bool => write!(f, "bool"),
            ElrondType::Int => write!(f, "int"),
            ElrondType::IntList => write!(f, "int list"),
            ElrondType::IntTree => write!(f, "int tree"),
            ElrondType::IntTreeI => write!(f, "int treei"),
            ElrondType::IntTreeB => write!(f, "int treeb"),
            ElrondType::Generic(s) => write!(f, "{s}"),
            ElrondType::Tuple(v) => {
                write!(f, "({})", v.iter().map(ToString::to_string).join(", "))
            }
            ElrondType::Arrow(t1, t2) => write!(f, "({t1} -> {t2})"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElrondSimpleExpr {
    Literal(ElrondType, ElrondLiteral),
    Var(ElrondType, String),
    Op(ElrondType, String, Vec<ElrondSimpleExpr>),
    Tuple(Vec<ElrondSimpleExpr>),
}

impl Display for ElrondSimpleExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondSimpleExpr::Literal(_ty, lit) => write!(f, "{lit}"),
            ElrondSimpleExpr::Var(_ty, name) => write!(f, "{name}"),
            ElrondSimpleExpr::Op(_ty, op, args) if op == "<=" || op == ">=" => {
                write!(f, "({} {} {})", args[0], op, args[1])
            }
            ElrondSimpleExpr::Op(_ty, op, args) => {
                write!(f, "{op} {}", args.iter().map(ToString::to_string).join(" "))
            }
            ElrondSimpleExpr::Tuple(t) => {
                write!(f, "({})", t.iter().map(ToString::to_string).join(", "))
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElrondPred {
    True,
    Atom(ElrondSimpleExpr),
    Implies(Box<ElrondPred>, Box<ElrondPred>),
    Ite(Box<ElrondPred>, Box<ElrondPred>, Box<ElrondPred>),
    Not(Box<ElrondPred>),
    And(Vec<ElrondPred>),
    Or(Vec<ElrondPred>),
    Iff(Box<ElrondPred>, Box<ElrondPred>),
}

impl ElrondPred {
    pub fn size(&self) -> u32 {
        match self {
            ElrondPred::True => 1,
            ElrondPred::Atom(_) => 1,
            ElrondPred::Iff(p1, p2) | ElrondPred::Implies(p1, p2) => 1 + p1.size() + p2.size(),
            ElrondPred::Ite(p1, p2, p3) => 1 + p1.size() + p2.size() + p3.size(),
            ElrondPred::Not(p) => 1 + p.size(),
            ElrondPred::And(p_vec) | ElrondPred::Or(p_vec) => {
                1 + p_vec.iter().map(ElrondPred::size).sum::<u32>()
            }
        }
    }
}

impl Display for ElrondPred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondPred::True => write!(f, "true"),
            ElrondPred::Atom(expr) => write!(f, "{expr}"),
            ElrondPred::Implies(lhs, rhs) => write!(f, "implies ({lhs}) ({rhs})"),
            ElrondPred::Ite(cond, lhs, rhs) => write!(f, "(if {cond} then {lhs} else {rhs})"),
            ElrondPred::Not(pred) => write!(f, "(!{pred})"),
            ElrondPred::And(preds) => write!(
                f,
                "({})",
                preds.iter().map(|p| format!("{}", p)).join(" && ")
            ),
            ElrondPred::Or(preds) => write!(
                f,
                "({})",
                preds.iter().map(|p| format!("{}", p)).join(" || ")
            ),
            ElrondPred::Iff(lhs, rhs) => write!(f, "iff ({lhs}) ({rhs})"),
        }
    }
}

pub enum ElrondOp {
    Eq,
    Le,
    Ge,
    Ne,
    Lt,
    Gt,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FreeVar(pub ElrondType, pub String);

impl Display for FreeVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FreeVar(ty, name) = self;
        write!(f, "{name} : {ty}")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ElrondTpedvar(pub AssertionType, pub String);

impl Display for ElrondTpedvar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ElrondTpedvar(ty, name) = self;
        write!(f, "{name} : {ty}")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ElrondForallFormula(pub Vec<FreeVar>, pub ElrondPred);

impl Display for ElrondForallFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ElrondForallFormula(vars, formula) = self;
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
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ElrondSpec(pub Vec<ElrondTpedvar>, pub ElrondForallFormula);

impl Display for ElrondSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ElrondSpec(vars, formula) = self;
        if vars.is_empty() {
            write!(f, "{formula}")
        } else {
            write!(
                f,
                "{} ⊢ {formula}",
                (vars.iter().map(ToString::to_string).join(","))
            )
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ElrondResult {
    Cex(Vec<Vec<(ElrondValue, String)>>),
    Result(Vec<(ElrondSpec, String)>),
}

impl Display for ElrondResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondResult::Cex(c) => {
                for (i, cex) in c.iter().enumerate() {
                    writeln!(f, "Cex {i}:")?;
                    for (val, name) in cex {
                        writeln!(f, "\t{name} -> {val}")?;
                    }
                }
                Ok(())
            }

            ElrondResult::Result(r) => {
                for (spec, name) in r {
                    writeln!(f, "\t{name} : {spec}")?;
                }
                Ok(())
            }
        }
    }
}

/// See translate/translate.ml -> TenvEngine -> known_preds
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum ElrondKnownPredicate {
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

impl Display for ElrondKnownPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElrondKnownPredicate::length => write!(f, "\"len\""),
            ElrondKnownPredicate::member => write!(f, "\"mem\""),
            ElrondKnownPredicate::head => write!(f, "\"hd\""),
            ElrondKnownPredicate::order => write!(f, "\"ord\""),
            ElrondKnownPredicate::left => write!(f, "\"left\""),
            ElrondKnownPredicate::right => write!(f, "\"right\""),
            ElrondKnownPredicate::sorted => write!(f, "\"sorted\""),
            ElrondKnownPredicate::once => write!(f, "\"once\""),
            ElrondKnownPredicate::para => write!(f, "\"para\""),
            ElrondKnownPredicate::ance => write!(f, "\"ance\""),
            ElrondKnownPredicate::root => write!(f, "\"root\""),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ElrondPredicates(pub Vec<ElrondKnownPredicate>);

impl Display for ElrondPredicates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "let preds = [| {} |]",
            self.0.iter().map(ToString::to_string).join("; ")
        )
    }
}

#[derive(Debug)]
pub struct ElrondAssertion {
    pub preds: ElrondPredicates,
    pub pre_spec: Option<ElrondSpec>,
    pub post_spec: ElrondSpec,
}

impl Display for ElrondAssertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ElrondAssertion {
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

impl ElrondSpec {
    fn spec_writer_for_file(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        kind: SpecKind,
    ) -> std::fmt::Result {
        let ElrondSpec(vars, formula) = self;
        write!(
            f,
            "let {kind} {} =\n  {}\n",
            vars.iter()
                .map(ToString::to_string)
                .map(|s| "(".to_string() + &s + ")")
                .join(" "),
            formula
        )
    }
}
