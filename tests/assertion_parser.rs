// Make tests to check that we can properly receive Elrond Messages
macro_rules! make_test {
    ($test_name:tt, $parser:ident, $str:expr) => {
        #[test]
        fn $test_name() {
            let parser = arwen_elrond_ipc::assertion_parser::$parser::new();
            let pred = $str.to_string();
            let parsed_pred = parser.parse(&pred).unwrap();
            insta::assert_display_snapshot!(parsed_pred);
        }
    };
}

make_test! {test_type_int, ARG_TPParser, "int"}

make_test! {test_type_bool, ARG_TPParser, "bool"}

make_test! {test_type_generic, ARG_TPParser, "T"}

make_test! {test_type_custom_stk, ARG_TPParser, "Customstk.t"}

make_test! {test_assertion_parser_1, AssertionParser, "mem l1 u"}

make_test! { test_assertion_parser_2, AssertionParser, "mem l1 u || mem l2 u"}

make_test! { test_assertion_parser_3, AssertionParser, "iff (mem l3 u) (mem l1 u || mem l2 u)"}

make_test! { test_assertion_parser_4, AssertionParser, "implies (hd l3 u) (hd l1 u || hd l2 u)"}

make_test! { test_assertion_parser_5, AssertionParser, "iff (mem l3 u) (mem l1 u || mem l2 u) && implies (hd l3 u) (hd l1 u || hd l2 u)"}

make_test! { test_post_spec_parser, PostSpecParser, "let post (l1 : Customstk.t) (l2 : Customstk.t) (l3 : Customstk.t) (u : int) =
iff (mem l3 u) (mem l1 u || mem l2 u) && implies (hd l3 u) (hd l1 u || hd l2 u)" }

/* make_test! { test_var, VAR_TUPLEParser, "x"}

make_test! { test_lit_true, LITParser, "true"}

make_test! { test_lit_false, LITParser, "false"}

make_test! { test_lit_int, LITParser, "0"}

make_test! { test_lit_int_2, LITParser, "50"} */

// Make tests to check that we can properly receive Elrond Messages
macro_rules! make_assertion_test {
    ($test_name:tt, $assertion_file:expr) => {
        #[test]
        fn $test_name() {
            let parser = arwen_elrond_ipc::assertion_parser::AssertionFileParser::new();

            let assertionfile = $assertion_file;
            let assertion = std::fs::read_to_string(assertionfile).unwrap();

            insta::assert_display_snapshot!(parser.parse(&assertion).unwrap());
        }
    };
}

make_assertion_test! {test_customstk_assertion1_parser, "ADT-Lemma-Discovery/data/customstk_assertion1.ml"}

make_assertion_test! {test_customstk_assertion2_parser, "ADT-Lemma-Discovery/data/customstk_assertion2.ml"}

make_assertion_test! {test_customstk_assertion3_parser, "ADT-Lemma-Discovery/data/customstk_assertion3.ml"}

make_assertion_test! {test_bankersq_assertion1_parser, "ADT-Lemma-Discovery/data/bankersq_assertion1.ml"}

make_assertion_test! {test_bankersq_assertion2_parser, "ADT-Lemma-Discovery/data/bankersq_assertion2.ml"}

make_assertion_test! {test_batchedq_assertion1_parser, "ADT-Lemma-Discovery/data/batchedq_assertion1.ml"}

make_assertion_test! {test_batchedq_assertion2_parser, "ADT-Lemma-Discovery/data/batchedq_assertion2.ml"}

make_assertion_test! {test_splayhp_assertion1_parser, "ADT-Lemma-Discovery/data/splayhp_assertion1.ml"}

make_assertion_test! {test_splayhp_assertion2_parser, "ADT-Lemma-Discovery/data/splayhp_assertion2.ml"}

make_assertion_test! {test_splayhp_assertion3_parser, "ADT-Lemma-Discovery/data/splayhp_assertion3.ml"}

make_assertion_test! {
    motivating_example_assertion_parser,
    "ADT-Lemma-Discovery/data/motivating_example_assertion.ml"
}
