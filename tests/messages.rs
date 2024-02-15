use arwen_elrond_ipc::{
    ipc::{ArwenMessage, ArwenSetup, ElrondKnownPredicate, ElrondPredicates},
    Elrond,
};

#[cfg(test)]
use arwen_elrond_ipc::ipc::{
    ElrondForallFormula, ElrondLiteral, ElrondMessage, ElrondPred, ElrondResult, ElrondSimpleExpr,
    ElrondSpec, ElrondTpedvar, ElrondType, ElrondValue, FreeVar,
};

// Make tests to check that we can properly receive Elrond Messages
macro_rules! make_test {
    ($test_name:tt, $sent_expr:expr, $recv_expr:expr) => {
        #[test]
        #[serial_test::serial]
        fn $test_name() {
            let mut e = Elrond::new();
            e.send_message(ArwenMessage::Test).unwrap();

            e.send_message($sent_expr).unwrap();

            assert!(dbg!(e.receive_message()) == dbg!($recv_expr));

            e.kill().unwrap();
        }
    };
}

make_test! {
    test_message,
    ArwenMessage::Message("hello World!".to_string()),
    ElrondMessage::Message("hello World!".to_string())
}

make_test! {
    test_l_value_empty,
    ArwenMessage::Message("L []".to_string()),
    ElrondMessage::Message(
        serde_json::to_string(&ElrondValue::L(vec![])).unwrap())
}

make_test! {
    test_l_value_one,
    ArwenMessage::Message("L [1]".to_string()),
    ElrondMessage::Message(
        serde_json::to_string(&ElrondValue::L(vec![1])).unwrap())
}

make_test! {
    test_l_value_two,
    ArwenMessage::Message("L [1, 0]".to_string()),
    ElrondMessage::Message(
        serde_json::to_string(&ElrondValue::L(vec![1, 0])).unwrap())
}

make_test! {
    test_i_value_zero,
    ArwenMessage::Message("I 0".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondValue::I(0)).unwrap())
}

make_test! {
    test_i_value_one,
    ArwenMessage::Message("I 1".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondValue::I(1)).unwrap())
}

make_test! {
    test_i_value_neg,
    ArwenMessage::Message("I -1".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondValue::I(-1)).unwrap())
}

make_test! {
    test_b_value_true,
    ArwenMessage::Message("B true".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondValue::B(true)).unwrap())
}

make_test! {
    test_b_value_false,
    ArwenMessage::Message("B false".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondValue::B(false)).unwrap())
}

make_test! {
    test_not_adt_value,
    ArwenMessage::Message("NotADt".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondValue::NotADt).unwrap())
}

make_test! {
    test_int_literal_1,
    ArwenMessage::Message("Int 1".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::Int(1)).unwrap())
}

make_test! {
    test_int_literal_0,
    ArwenMessage::Message("Int 0".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::Int(0)).unwrap())
}

make_test! {
    test_int_literal_neg1,
    ArwenMessage::Message("Int -1".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::Int(-1)).unwrap())
}

make_test! {
    test_bool_literal_true,
    ArwenMessage::Message("Bool true".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::Bool(true)).unwrap())
}

make_test! {
    test_bool_literal_false,
    ArwenMessage::Message("Bool false".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::Bool(false)).unwrap())
}

make_test! {
    test_int_list_literal_empty,
    ArwenMessage::Message("IntList []".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::IntList(vec![])).unwrap())
}

make_test! {
    test_int_list_literal_one,
    ArwenMessage::Message("IntList [1]".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::IntList(vec![1])).unwrap())
}

make_test! {
    test_int_list_literal_two,
    ArwenMessage::Message("IntList [1, 0]".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondLiteral::IntList(vec![1, 0])).unwrap())
}

make_test! {
    test_b_type,
    ArwenMessage::Message("Bool".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondType::Bool).unwrap())
}

make_test! {
    test_i_type,
    ArwenMessage::Message("Int".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondType::Int).unwrap())
}

make_test! {
    test_il_type,
    ArwenMessage::Message("IntList".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondType::IntList).unwrap())
}

make_test! {
    test_itt_type,
    ArwenMessage::Message("IntTree".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondType::IntTree).unwrap())
}

make_test! {
    test_itti_type,
    ArwenMessage::Message("IntTreeI".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondType::IntTreeI).unwrap())
}

make_test! {
    test_itb_type,
    ArwenMessage::Message("IntTreeB".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondType::IntTreeB).unwrap())
}

make_test! {
    test_l_simple_expr_zero,
    ArwenMessage::Message("Lit i 1".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1))).unwrap())
}

make_test! {
    test_l_simple_expr_one,
    ArwenMessage::Message("Lit b true".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSimpleExpr::Literal(ElrondType::Bool, ElrondLiteral::Bool(true))).unwrap())
}

make_test! {
    test_l_simple_expr_two,
    ArwenMessage::Message("Lit il [1]".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSimpleExpr::Literal(ElrondType::IntList, ElrondLiteral::IntList(vec![1]))).unwrap())
}

make_test! {
    test_var_simple_expr,
    ArwenMessage::Message("Var i x".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSimpleExpr::Var(ElrondType::Int, "x".to_string())).unwrap())
}

make_test! {
    test_op_simple_expr_empty,
    ArwenMessage::Message("Op b t []".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSimpleExpr::Op(ElrondType::Bool, "t".to_string(), vec![])).unwrap())
}

make_test! {
    test_op_simple_expr,
    ArwenMessage::Message("Op i + [Lit i 1, Lit i 2]".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSimpleExpr::Op(ElrondType::Int, "+".to_string(), vec![ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)), ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2))])).unwrap())
}

make_test! {
    test_true_pred,
    ArwenMessage::Message("Pred True".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::True).unwrap())
}

make_test! {
    test_atom_pred,
    ArwenMessage::Message("Pred Atom (Lit i 1)".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))).unwrap())
}

make_test! {
    test_implies_pred,
    ArwenMessage::Message("Pred Implies (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Implies(Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))))).unwrap())
}

make_test! {
    test_implies_true,
    ArwenMessage::Message("Pred Implies True (Pred Atom (Lit i 2))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Implies(Box::new(ElrondPred::True) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))))).unwrap())
}

make_test! {
    test_ite_pred,
    ArwenMessage::Message("Pred Ite (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2)) (Pred Atom (Lit i 3))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Ite(Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))), Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(3)))))).unwrap())
}

make_test! {
    test_ite_implies_pred,
    ArwenMessage::Message("Pred Ite (Pred Implies (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2))) (Pred Atom (Lit i 3)) (Pred Atom (Lit i 4))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Ite(Box::new(ElrondPred::Implies(Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))))) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(3)))) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(4)))))).unwrap())
}

make_test! {
    test_not_pred,
    ArwenMessage::Message("Pred Not (Pred Atom (Lit i 1))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Not(Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))))).unwrap())
}

make_test! {
    test_not_ite,
    ArwenMessage::Message("Pred Not (Pred Ite (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2)) (Pred Atom (Lit i 3)))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Not(Box::new(ElrondPred::Ite(Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))), Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(3)))))))).unwrap())
}

make_test! {
    test_and_not_pred,
    ArwenMessage::Message("Pred And [Pred Atom (Lit i 1), Pred Not (Pred Atom (Lit i 2))]".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::And(vec![ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1))), ElrondPred::Not(Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))))])).unwrap())
}

make_test! {
    test_or_and_pred,
    ArwenMessage::Message("Pred Or [Pred And [Pred Atom (Lit i 2), Pred Atom (Lit i 3)]], Pred Atom (Lit i 1)".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPred::Or(vec![ElrondPred::And(vec![ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2))), ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(3)))]) , ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1)))])).unwrap())
}

make_test! {
    test_iff_or_pred,
    ArwenMessage::Message("Pred Iff (Pred Or [Pred Atom (Lit i 1), Pred Atom (Lit i 2)]) (Pred Atom (Lit i 3))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&
        ElrondPred::Iff(Box::new(ElrondPred::Or(vec![ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1))), ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(2)))])) , Box::new(ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(3))))
    )).unwrap())
}

make_test! {
    test_free_var,
    ArwenMessage::Message("FreeVar i x".to_string()),
    ElrondMessage::Message(serde_json::to_string(&FreeVar(ElrondType::Int, "x".to_string())).unwrap())
}

make_test! {
    test_elrondtpedvar,
    ArwenMessage::Message("ElrondTpedvar i x".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondTpedvar(ElrondType::Int, "x".to_string())).unwrap())
}

make_test! {
    test_forall_empty,
    ArwenMessage::Message("Forall [] (Pred Atom (Lit i 1))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondForallFormula(vec![], ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1))))).unwrap())
}

make_test! {
    test_forall_one,
    ArwenMessage::Message("Forall [i x] (Pred Atom (Lit i 1))".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondForallFormula(vec![FreeVar(ElrondType::Int, "x".to_string())], ElrondPred::Atom(ElrondSimpleExpr::Literal(ElrondType::Int, ElrondLiteral::Int(1))))).unwrap())
}

make_test! {
    test_spec_empty,
    ArwenMessage::Message("Spec [] ([] Pred True)".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSpec(vec![], ElrondForallFormula(vec![], ElrondPred::True))).unwrap())
}

make_test! {
    test_spec_one,
    ArwenMessage::Message("Spec [i x] ([] Pred True)".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSpec(vec![ElrondTpedvar(ElrondType::Int, "x".to_string())], ElrondForallFormula(vec![], ElrondPred::True))).unwrap())
}

make_test! {
    test_spec_many,
    ArwenMessage::Message("Spec [i x, il y] ([i x] Pred True)".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondSpec(vec![ElrondTpedvar(ElrondType::Int, "x".to_string()), ElrondTpedvar(ElrondType::IntList, "y".to_string())], ElrondForallFormula(vec![FreeVar(ElrondType::Int, "x".to_string())], ElrondPred::True))).unwrap())
}

make_test! {
    test_result_cex_single,
    ArwenMessage::Message("Result (Cex [[x, [L [1; 2]]])".to_string()),
    ElrondMessage::Message(serde_json::to_string(&ElrondResult::Cex(vec![vec![( ElrondValue::L(vec![1, 2]), "x".to_string())]])).unwrap())
}

make_test! {
    test_result_spec_single,
    ArwenMessage::Message("Result (Spec [([i x] ([] Pred True)), x)]".to_string()),
    ElrondMessage::Message(serde_json::to_string(&
        ElrondResult::Result(vec![(
            ElrondSpec(vec![ElrondTpedvar(ElrondType::Int, "x".to_string())], ElrondForallFormula(vec![], ElrondPred::True)), "x".to_string())
        ])
    ).unwrap())
}

make_test! {
    test_elrond_predicates,
    ArwenMessage::Message(serde_json::to_string(&ElrondPredicates(vec![ElrondKnownPredicate::list_member, ElrondKnownPredicate::list_order])).unwrap()),
    ElrondMessage::Message(serde_json::to_string(&ElrondPredicates(vec![ElrondKnownPredicate::list_member, ElrondKnownPredicate::list_order])).unwrap())
}

make_test! {
    test_setup,
    {
        let dir = "ADT-Lemma-Discovery".to_string();
        let sourcefile = dir.clone() + "/data/customstk.ml";
        let assertionfile = dir + "/data/customstk_assertion1.ml";
        let outputdir = "customstk_out".to_string();
        let client_name = "concat".to_string();
        let predicates = ElrondPredicates(vec![        ElrondKnownPredicate::list_member,
        ElrondKnownPredicate::list_head,]);
        ArwenMessage::Setup(ArwenSetup {sourcefile, assertionfile, outputdir, client_name, predicates})
    },
    ElrondMessage::Message("ADT-Lemma-Discovery/data/customstk.ml ADT-Lemma-Discovery/data/customstk_assertion1.ml customstk_out concat list_member list_head".to_string())
}
