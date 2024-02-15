module SpecAbd = Inference.SpecAbduction
module Vc = Language.SpecAst
open Printf
open Ipc

(* type infer_action = InferConsistent (*| InferFull | InferWeakening*) *)

(* let dir = "ADT-Lemma-Discovery"
   (* let action = InferConsistent *)
   let sourcefile = String.cat dir "/data/customstk.ml"
   let assertionfile = String.cat dir "/data/customstk_assertion1.ml"
   let outputdir = "customstk_out" *)

let start (* action *) sourcefile assertionfile outputdir client_name preds =
  let ctx = Main_lib.init () in
  let source = Main_lib.parse sourcefile in
  let assertion = Main_lib.parse assertionfile in

  let mii, vc, holes, _preds, spectab =
    let ( _client_name,
          (signame, tpnames),
          fnames,
          raw_funcm,
          (intp, outtp),
          client ) =
      Translate.parse_source source
    in

    let () = eprintf "signame:=%s\n" signame in
    (*
    let _ = printf "client:=\n%s\n" (Main_lib.Ast.layout client) in *)
    (* let _ = printf "raw_funcm:=\n%s\n" (Translate.layout_raw_funcm raw_funcm) in *)
    let _ =
      Utils.StrMap.to_kv_list raw_funcm
      |> Utils.List.to_string (fun (x, (y, z)) ->
             sprintf "key:=(%s, %s)\n" x
               (String.cat (String.concat "," y) (String.concat "," z)))
      |> eprintf "raw_funcm:=\n%s\n"
    in

    (* let _ = printf "signame:=\n%s\n" signame in *)
    (* let _ = printf "tpnames:=\n%s\n" (List.to_string T.layouttvar tpnames) in *)

    (* let _ = printf "intp:=\n%s\n" (List.to_string T.layouttvar intp) in *)
    (* let _ = printf "outtp:=\n%s\n" (List.to_string T.layouttvar outtp) in *)
    let init_tenv =
      Translate.TenvEngine.make_tenv signame tpnames Utils.StrMap.empty
        Utils.StrMap.empty
    in
    let imp_map = Translate.Impmap.l_to_map (Translate.find signame) in
    let preds, asst, spectab =
      Translate.parse_assertion client_name init_tenv (intp @ outtp) assertion
    in

    let tenv = Translate.TenvEngine.renew_raw_funcm init_tenv raw_funcm in
    (* let () = Translate.TenvEngine.print_tenv tenv in *)
    let tenv, uinputs, body = Translate.parse_client tenv client in
    (* let () = Translate.TenvEngine.print_tenv tenv in *)
    let vc, uoutputs = Translate.body_vc_gen client_name tenv asst body in
    (* let _ = printf "body:=\n%s\n" (Vc.layout vc) in *)
    (* let _ = printf "vc:%s\n" (Vc.vc_layout vc); raise @@ InterExn "end" in *)
    let preds = Translate.TenvEngine.all_preds tenv preds in
    let holes = Translate.make_holes fnames tenv.funcm imp_map in
    let uvars = Vc.get_uvars vc in
    (* let _ = printf "%s\n" (List.to_string T.layouttvar uvars); raise @@ InterExn "end" in *)
    (* let _ = printf "%s\n" (List.to_string T.layouttvar uinputs) in
     * let _ = printf "%s\n" (List.to_string T.layouttvar uoutputs) in
     * let _ = raise @@ InterExn "end" in *)
    let open Inference.SpecAbduction in
    let mii =
      let args = List.map SE.from_tpedvar (uinputs @ uoutputs) in
      let upost =
        match asst with
        | NoPre (postname, _) -> Vc.SpecApply (postname, args)
        | HasPre (prename, _, postname, _) ->
            Vc.Implies
              (Vc.SpecApply (prename, args), Vc.SpecApply (postname, args))
      in
      {
        upost;
        uvars;
        uinputs;
        uoutputs;
        uprog = Utils.StrMap.find "trans::imp_map" imp_map client_name;
      }
    in
    (mii, vc, holes, preds, spectab)
  in

  (* I want to print out mii *)

  (* `vc` *)
  let () = Main_lib.Ast.vc_layout vc |> eprintf "VC: %s" in

  (* `spectab` is a list of specs. Each spec is a list of predicates. Each predicate *)

  (* `holes` are a list of library functions and their implementations which is hardcoded for each benchmark *)
  let _fnames =
    [
      ("Customstk.push", "Customstk.is_empty", "Customstk.top", "Customstk.tail");
    ]
  in
  let _imp_map = Translate.Impmap.l_to_map (Translate.find "Customstk") in

  (* let tenv = Translate.TenvEngine.renew_raw_funcm init_tenv raw_funcm in
     let () = Translate.TenvEngine.print_tenv tenv in
     let tenv, uinputs, body = Translate.parse_client tenv client in *)

  (* let holes = Translate.make_holes fnames tenv.funcm imp_map in *)
  let () =
    eprintf "\n\nHoles: %s\n\n"
      (List.map (fun (({ name; args } : Language.Helper.hole), _) -> name) holes
      |> String.concat " ")
  in

  (* let preds = Translate.TenvEngine.all_preds tenv preds in *)
  let r () = SpecAbd.do_consistent outputdir ctx mii vc spectab holes preds 1 in
  let res = Utils.time r in

  res |> fst |> into_elrondResult |> into_elrondMessage
  |> elrondMessage_to_yojson |> Yojson.Safe.to_string |> print_endline;
  ()

(* match res with
   | SpecAbd.Cex _, delta_time ->
       eprintf "Failed with Cex in %f(s)!\n" delta_time
   | SpecAbd.Result spectab, delta_time ->
       let mode_str =
         (* match action with
            (*  | InferFull -> "Full" *)
            | InferConsistent -> *)
         "Consistent"
         (* | InferWeakening -> "Weakening" *)
       in
       let () = Main_lib.Ast.eprint_spectable spectab in
       eprintf "%s inference Succeeded in %f(s)!\n" mode_str delta_time *)

(* Take an elrond thing, conver it to yojson, print it out and then wrap it in a message over the wire *)
let test_helper elrondThing thing_to_yojson =
  let x = elrondThing |> thing_to_yojson in
  eprintf "Show return message: %s\n" (Yojson.Safe.show x);
  x |> Yojson.Safe.to_string |> fun x ->
  Message x |> elrondMessage_to_yojson |> Yojson.Safe.to_string |> print_endline

let test_message_helper str =
  let x = Message str |> elrondMessage_to_yojson in
  eprintf "Show return message: %s\n" (Yojson.Safe.show x);
  print_endline (x |> Yojson.Safe.to_string)

let test_loop () =
  try
    while true do
      let message_str = input_line stdin in
      eprintf "Received message: %s\n" message_str;
      let message_json = Yojson.Safe.from_string message_str in
      eprintf "Parsed message:   %s\n" (Yojson.Safe.to_string message_json);

      let test_message : Ipc.arwenMessage =
        Setup
          {
            sourcefile = "ADT-Lemma-Discovery/data/customstk.ml";
            assertionfile = "ADT-Lemma-Discovery/data/customstk_assertion1.ml";
            outputdir = "customstk_out";
            client_name = "concat";
            predicates = [ "list_member"; "list_order" ];
          }
      in
      let test_message_json = arwenMessage_to_yojson test_message in
      eprintf "Test message:     %s\n" (Yojson.Safe.to_string test_message_json);
      let message = arwenMessage_of_yojson message_json in
      match message with
      | Ok (Message "hello World!") -> test_message_helper "hello World!"
      | Ok (Message "L []") -> test_helper (L []) elrondValue_to_yojson
      | Ok (Message "L [1]") -> test_helper (L [ 1 ]) elrondValue_to_yojson
      | Ok (Message "L [1, 0]") ->
          test_helper (L [ 1; 0 ]) elrondValue_to_yojson
      | Ok (Message "I 0") -> test_helper (I 0) elrondValue_to_yojson
      | Ok (Message "I 1") -> test_helper (I 1) elrondValue_to_yojson
      | Ok (Message "I -1") -> test_helper (I (-1)) elrondValue_to_yojson
      | Ok (Message "B true") -> test_helper (B true) elrondValue_to_yojson
      | Ok (Message "B false") -> test_helper (B false) elrondValue_to_yojson
      | Ok (Message "NotADt") -> test_helper NotADt elrondValue_to_yojson
      | Ok (Message "Int 1") ->
          test_helper (Int 1 : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "Int 0") ->
          test_helper (Int 0 : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "Int -1") ->
          test_helper (Int (-1) : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "Bool true") ->
          test_helper (Bool true : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "Bool false") ->
          test_helper (Bool false : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "IntList []") ->
          test_helper (IntList [] : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "IntList [1]") ->
          test_helper (IntList [ 1 ] : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "IntList [1, 0]") ->
          test_helper (IntList [ 1; 0 ] : elrondLiteral) elrondLiteral_to_yojson
      | Ok (Message "Bool") ->
          test_helper (Bool : elrondType) elrondType_to_yojson
      | Ok (Message "Int") ->
          test_helper (Int : elrondType) elrondType_to_yojson
      | Ok (Message "IntList") ->
          test_helper (IntList : elrondType) elrondType_to_yojson
      | Ok (Message "IntTree") ->
          test_helper (IntTree : elrondType) elrondType_to_yojson
      | Ok (Message "IntTreeI") ->
          test_helper (IntTreeI : elrondType) elrondType_to_yojson
      | Ok (Message "IntTreeB") ->
          test_helper (IntTreeB : elrondType) elrondType_to_yojson
      | Ok (Message "Lit i 1") ->
          test_helper
            (Literal ((Int : elrondType), (Int 1 : elrondLiteral)))
            elrondSimpleExpr_to_yojson
      | Ok (Message "Lit b true") ->
          test_helper
            (Literal ((Bool : elrondType), (Bool true : elrondLiteral)))
            elrondSimpleExpr_to_yojson
      | Ok (Message "Lit il [1]") ->
          test_helper
            (Literal ((IntList : elrondType), (IntList [ 1 ] : elrondLiteral)))
            elrondSimpleExpr_to_yojson
      | Ok (Message "Var i x") ->
          test_helper (Var (Int, "x")) elrondSimpleExpr_to_yojson
      | Ok (Message "Op b t []") ->
          test_helper (Op (Bool, "t", [])) elrondSimpleExpr_to_yojson
      | Ok (Message "Op i + [Lit i 1, Lit i 2]") ->
          let x =
            Op
              ( Int,
                "+",
                [
                  Literal ((Int : elrondType), (Int 1 : elrondLiteral));
                  Literal ((Int : elrondType), (Int 2 : elrondLiteral));
                ] )
          in
          test_helper x elrondSimpleExpr_to_yojson
      | Ok (Message "Pred True") -> test_helper True elrondPred_to_yojson
      | Ok (Message "Pred Atom (Lit i 1)") ->
          test_helper
            (Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))))
            elrondPred_to_yojson
      | Ok (Message "Pred Implies (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2))")
        ->
          test_helper
            (Implies
               ( Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))),
                 Atom (Literal ((Int : elrondType), (Int 2 : elrondLiteral))) ))
            elrondPred_to_yojson
      | Ok (Message "Pred Implies True (Pred Atom (Lit i 2))") ->
          test_helper
            (Implies
               ( True,
                 Atom (Literal ((Int : elrondType), (Int 2 : elrondLiteral))) ))
            elrondPred_to_yojson
      | Ok
          (Message
            "Pred Ite (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2)) (Pred Atom \
             (Lit i 3))") ->
          test_helper
            (Ite
               ( Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))),
                 Atom (Literal ((Int : elrondType), (Int 2 : elrondLiteral))),
                 Atom (Literal ((Int : elrondType), (Int 3 : elrondLiteral))) ))
            elrondPred_to_yojson
      | Ok
          (Message
            "Pred Ite (Pred Implies (Pred Atom (Lit i 1)) (Pred Atom (Lit i \
             2))) (Pred Atom (Lit i 3)) (Pred Atom (Lit i 4))") ->
          test_helper
            (Ite
               ( Implies
                   ( Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))),
                     Atom
                       (Literal ((Int : elrondType), (Int 2 : elrondLiteral)))
                   ),
                 Atom (Literal ((Int : elrondType), (Int 3 : elrondLiteral))),
                 Atom (Literal ((Int : elrondType), (Int 4 : elrondLiteral))) ))
            elrondPred_to_yojson
      | Ok (Message "Pred Not (Pred Atom (Lit i 1))") ->
          test_helper
            (Not (Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral)))))
            elrondPred_to_yojson
      | Ok
          (Message
            "Pred Not (Pred Ite (Pred Atom (Lit i 1)) (Pred Atom (Lit i 2)) \
             (Pred Atom (Lit i 3)))") ->
          test_helper
            (Not
               (Ite
                  ( Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))),
                    Atom (Literal ((Int : elrondType), (Int 2 : elrondLiteral))),
                    Atom (Literal ((Int : elrondType), (Int 3 : elrondLiteral)))
                  )))
            elrondPred_to_yojson
      | Ok
          (Message
            "Pred And [Pred Atom (Lit i 1), Pred Not (Pred Atom (Lit i 2))]") ->
          test_helper
            (And
               [
                 Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral)));
                 Not
                   (Atom (Literal ((Int : elrondType), (Int 2 : elrondLiteral))));
               ])
            elrondPred_to_yojson
      | Ok
          (Message
            "Pred Or [Pred And [Pred Atom (Lit i 2), Pred Atom (Lit i 3)]], \
             Pred Atom (Lit i 1)") ->
          test_helper
            (Or
               [
                 And
                   [
                     Atom
                       (Literal ((Int : elrondType), (Int 2 : elrondLiteral)));
                     Atom
                       (Literal ((Int : elrondType), (Int 3 : elrondLiteral)));
                   ];
                 Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral)));
               ])
            elrondPred_to_yojson
      | Ok
          (Message
            "Pred Iff (Pred Or [Pred Atom (Lit i 1), Pred Atom (Lit i 2)]) \
             (Pred Atom (Lit i 3))") ->
          test_helper
            (Iff
               ( Or
                   [
                     Atom
                       (Literal ((Int : elrondType), (Int 1 : elrondLiteral)));
                     Atom
                       (Literal ((Int : elrondType), (Int 2 : elrondLiteral)));
                   ],
                 Atom (Literal ((Int : elrondType), (Int 3 : elrondLiteral))) ))
            elrondPred_to_yojson
      | Ok (Message "ElrondTpedvar i x") ->
          test_helper (Int, "x") elrondTpedvar_to_yojson
      | Ok (Message "FreeVar i x") -> test_helper (Int, "x") free_var_to_yojson
      | Ok (Message "Forall [] (Pred Atom (Lit i 1))") ->
          let x =
            ([], Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))))
          in
          test_helper x elrondForallFormula_to_yojson
      | Ok (Message "Forall [i x] (Pred Atom (Lit i 1))") ->
          let x =
            ( [ (Int, "x") ],
              Atom (Literal ((Int : elrondType), (Int 1 : elrondLiteral))) )
          in
          test_helper x elrondForallFormula_to_yojson
      | Ok (Message "Spec [] ([] Pred True)") ->
          test_helper ([], ([], True)) elrondSpec_to_yojson
      | Ok (Message "Spec [i x] ([] Pred True)") ->
          test_helper ([ (Int, "x") ], ([], True)) elrondSpec_to_yojson
      | Ok (Message "Spec [i x, il y] ([i x] Pred True)") ->
          let x = ([ (Int, "x"); (IntList, "y") ], ([ (Int, "x") ], True)) in
          test_helper x elrondSpec_to_yojson
      | Ok (Message "Result (Cex [[x, [L [1; 2]]])") ->
          test_helper (Cex [ [ (L [ 1; 2 ], "x") ] ]) elrondResult_to_yojson
      | Ok (Message "Result (Spec [([i x] ([] Pred True)), x)]") ->
          let x : elrondResult =
            Result [ (([ (Int, "x") ], ([], True)), "x") ]
          in
          test_helper x elrondResult_to_yojson
      | Ok (Message ("[\"list_member\",\"list_order\"]" as s)) ->
          let x : elrondPredicates = [ "list_member"; "list_order" ] in
          assert (
            s |> Yojson.Safe.from_string |> elrondPredicates_of_yojson
            |> Result.get_ok = x);
          test_helper x elrondPredicates_to_yojson
      | Ok
          (Setup
            { sourcefile; assertionfile; outputdir; client_name; predicates })
        ->
          test_message_helper
            (String.concat " "
               (List.append
                  [ sourcefile; assertionfile; outputdir; client_name ]
                  predicates))
      | Ok _ ->
          eprintf "Erroring out with unplanned case";
          exit 1
      | Error e ->
          eprintf "Erroring out with %s\n" e;
          exit 1
    done
  with End_of_file -> ()

(* todo maybe from_channel instead?*)
let start_loop () =
  let message_str = input_line stdin in
  eprintf "Received message: %s\n" message_str;
  let message_json = Yojson.Safe.from_string message_str in
  let message = arwenMessage_of_yojson message_json in
  match message with
  | Ok (Setup { sourcefile; assertionfile; outputdir; client_name; predicates })
    ->
      start sourcefile assertionfile outputdir client_name predicates
  | Ok _ ->
      eprintf "Erroring out with unplanned case : %s" message_str;
      exit 1
  | Error e ->
      eprintf "Erroring out with error: %s : %s" e message_str;
      exit 1

let run () =
  let init_message = input_line stdin in
  eprintf "Initial message: %s\n" init_message;

  (*   eprintf "Expected message: %s\n" (arwenMessage_to_yojson Test |> Yojson.Safe.to_string); *)
  Yojson.Safe.from_string init_message
  |> Yojson.Safe.to_string |> eprintf "Parsed to %s\n";

  (* let ymsg = Yojson.Safe.from_string init_message in
     print_newline (); *)
  (* let () = Format.printf "Parsed to %s\n" (Yojson.Safe.show ymsg) in

     let () = Format.printf "Expected to be %s\n" (Yojson.Safe.show(elrondMessage_to_yojson Start)) in *)
  match Yojson.Safe.from_string init_message |> arwenMessage_of_yojson with
  | Ok Test -> test_loop ()
  | Ok Start -> start_loop ()
  | Ok _ ->
      eprintf "Error recieved unexpected %s\n" init_message;
      exit 1
  | Error e ->
      eprintf "Erroring out with %s\n" e;
      exit 1

let () = run ()
