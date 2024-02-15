use arwen_elrond_ipc::{
    ipc::{ArwenMessage, ArwenSetup, ElrondKnownPredicate, ElrondPredicates},
    Elrond,
};

#[cfg(test)]
use arwen_elrond_ipc::ipc::{
    ElrondAssertion, ElrondForallFormula, ElrondPred, ElrondSimpleExpr, ElrondSpec, ElrondTpedvar,
    ElrondType,
};

fn main() {
    let mut e = Elrond::new();

    e.send_message(ArwenMessage::Start).unwrap();

    let dir = "ADT-Lemma-Discovery".to_string();
    // Source ocaml file
    let sourcefile = dir.clone() + "/data/customstk.ml";
    let assertionfile = dir + "/data/customstk_assertion1.ml";
    let outputdir = "customstk_out".to_string();

    // AKA the name of the function being verified
    let client_name = "concat".to_string();

    // fnames are the component names

    // This is a little tricky, you will get weird results if some of these are missing
    let predicates = ElrondPredicates(vec![
        ElrondKnownPredicate::member,
        ElrondKnownPredicate::head,
    ]);

    // todo build in a check such that only the list of provided predicates are used in any specifications

    e.send_message(ArwenMessage::Setup(ArwenSetup {
        sourcefile,
        assertionfile,
        outputdir,
        client_name,
        predicates,
    }))
    .unwrap();

    println!("{}", e.receive_message());

    e.kill().unwrap();
}

#[test]
fn test_customstk_assertion1() {
    // Read in the file called "ADT-Lemma-Discovery/data/customstk_assertion1.ml"
    let assertionfile = "ADT-Lemma-Discovery/data/customstk_assertion1.ml";
    let assertion = std::fs::read_to_string(assertionfile).unwrap();

    let generic_type = ElrondType::Generic("Customstk.t".to_string());

    let my_assertion = ElrondAssertion {
        preds: ElrondPredicates(vec![
            ElrondKnownPredicate::member,
            ElrondKnownPredicate::head,
        ]),
        pre_spec: None,
        post_spec: ElrondSpec(
            vec![
                ElrondTpedvar(generic_type.clone(), "l1".to_string()),
                ElrondTpedvar(generic_type.clone(), "l2".to_string()),
                ElrondTpedvar(generic_type.clone(), "l3".to_string()),
                ElrondTpedvar(ElrondType::Int, "u".to_string()),
            ],
            ElrondForallFormula(
                vec![],
                ElrondPred::And(vec![
                    ElrondPred::Iff(
                        Box::new(ElrondPred::Atom(ElrondSimpleExpr::Op(
                            ElrondType::Bool,
                            "mem".to_string(),
                            vec![
                                ElrondSimpleExpr::Var(generic_type.clone(), "l3".to_string()),
                                ElrondSimpleExpr::Var(ElrondType::Int, "u".to_string()),
                            ],
                        ))),
                        Box::new(ElrondPred::Or(vec![
                            ElrondPred::Atom(ElrondSimpleExpr::Op(
                                ElrondType::Bool,
                                "mem".to_string(),
                                vec![
                                    ElrondSimpleExpr::Var(generic_type.clone(), "l1".to_string()),
                                    ElrondSimpleExpr::Var(ElrondType::Int, "u".to_string()),
                                ],
                            )),
                            ElrondPred::Atom(ElrondSimpleExpr::Op(
                                ElrondType::Bool,
                                "mem".to_string(),
                                vec![
                                    ElrondSimpleExpr::Var(generic_type.clone(), "l2".to_string()),
                                    ElrondSimpleExpr::Var(ElrondType::Int, "u".to_string()),
                                ],
                            )),
                        ])),
                    ),
                    ElrondPred::Implies(
                        Box::new(ElrondPred::Atom(ElrondSimpleExpr::Op(
                            ElrondType::Bool,
                            "hd".to_string(),
                            vec![
                                ElrondSimpleExpr::Var(generic_type.clone(), "l3".to_string()),
                                ElrondSimpleExpr::Var(ElrondType::Int, "u".to_string()),
                            ],
                        ))),
                        Box::new(ElrondPred::Or(vec![
                            ElrondPred::Atom(ElrondSimpleExpr::Op(
                                ElrondType::Bool,
                                "hd".to_string(),
                                vec![
                                    ElrondSimpleExpr::Var(generic_type.clone(), "l1".to_string()),
                                    ElrondSimpleExpr::Var(ElrondType::Int, "u".to_string()),
                                ],
                            )),
                            ElrondPred::Atom(ElrondSimpleExpr::Op(
                                ElrondType::Bool,
                                "hd".to_string(),
                                vec![
                                    ElrondSimpleExpr::Var(generic_type, "l2".to_string()),
                                    ElrondSimpleExpr::Var(ElrondType::Int, "u".to_string()),
                                ],
                            )),
                        ])),
                    ),
                ]),
            ),
        ),
    };
    insta::assert_display_snapshot!(assertion, my_assertion);
}
