open Yojson

(* https://stackoverflow.com/questions/70917952/how-can-i-change-how-ppx-yojson-conv-represents-variants *)
let rec to_assoc : [> Safe.t ] -> Safe.t as 'a = function
  (* todo is this true only for strings or also other things? *)
  | `List [ `String "Pair"; `List [ key; value ] ] ->
      `Assoc [ ("Pair", `List [ key; to_assoc value ]) ]
  | `List (`String "Pair" :: _) -> failwith "Not possible"
  | `List (`String key :: []) -> `String key
  | `List [ `String key; elem ] -> `Assoc [ (key, to_assoc elem) ]
  | `List (`String key :: l) -> `Assoc [ (key, `List (List.map to_assoc l)) ]
  | `List l -> `List (List.map to_assoc l)
  | `Assoc [ (key, l) ] -> `Assoc [ (key, to_assoc l) ]
  | x -> x

(* todo does this need to be rec? *)
let rec to_list = function
  | `Assoc [ (key, `List l) ] -> `List (`String key :: l)
  | `Assoc [ (key, `String l) ] -> `List [ `String key; `String l ]
  | `Assoc [ (key, l) ] -> `List [ `String key; l ]
  | `String key -> `List (`String key :: [])
  | x -> x

type elrondValue =
  | L of int list
  (* | T of int Utils.Tree.t *)
  | I of int
  | B of bool
  (* | TI of (int, int) Utils.LabeledTree.t
     | TB of (int, bool) Utils.LabeledTree.t *)
  | NotADt
[@@deriving yojson]

type elrondLiteral = Int of int | Bool of bool | IntList of int list
(*   | IntTree of int Utils.Tree.t *)
[@@deriving yojson]

type elrondType = Bool | Int | IntList | IntTree | IntTreeI | IntTreeB
[@@deriving yojson]

type elrondSimpleExpr =
  | Literal of elrondType * elrondLiteral
  | Var of elrondType * string
  | Op of elrondType * string * elrondSimpleExpr list
[@@deriving yojson]

type elrondPred =
  | True
  | Atom of elrondSimpleExpr
  | Implies of elrondPred * elrondPred
  | Ite of elrondPred * elrondPred * elrondPred
  | Not of elrondPred
  | And of elrondPred list
  | Or of elrondPred list
  | Iff of elrondPred * elrondPred
[@@deriving yojson]

type free_var = elrondType * string [@@deriving yojson]
type elrondTpedvar = elrondType * string [@@deriving yojson]
type elrondForallFormula = free_var list * elrondPred [@@deriving yojson]
type elrondSpec = elrondTpedvar list * elrondForallFormula [@@deriving yojson]

type elrondResult =
  | Cex of (elrondValue * string) list list
  | Result of (elrondSpec * string) list
[@@deriving yojson]

type elrondMessage = Message of string | Result of elrondResult
[@@deriving yojson]

type elrondPredicates = string list [@@deriving yojson]

type arwenSetup = {
  sourcefile : string;
  assertionfile : string;
  outputdir : string;
  client_name : string;
  predicates : elrondPredicates;
}
[@@deriving yojson]

type arwenMessage = Start | Test | Message of string | Setup of arwenSetup
[@@deriving yojson]

let dbg yojson =
  Printf.eprintf "Debuged yojson: %s\n" (Yojson.Safe.show yojson);
  yojson

let arwenMessage_to_yojson x = to_assoc (arwenMessage_to_yojson x)
let arwenMessage_of_yojson x = arwenMessage_of_yojson (to_list x)
let elrondMessage_to_yojson x = elrondMessage_to_yojson x |> to_assoc
let elrondMessage_of_yojson x = elrondMessage_of_yojson (to_list x)
let elrondValue_to_yojson x = elrondValue_to_yojson x |> to_assoc
let elrondValue_of_yojson x = elrondValue_of_yojson (to_list x)
let elrondLiteral_to_yojson x = elrondLiteral_to_yojson x |> to_assoc
let elrondLiteral_of_yojson x = elrondLiteral_of_yojson (to_list x)
let elrondType_to_yojson x = elrondType_to_yojson x |> to_assoc
let elrondType_of_yojson x = elrondType_of_yojson (to_list x)
let elrondSimpleExpr_to_yojson x = elrondSimpleExpr_to_yojson x |> to_assoc
let elrondSimpleExpr_of_yojson x = elrondSimpleExpr_of_yojson (to_list x)
let elrondPred_to_yojson x = elrondPred_to_yojson x |> to_assoc
let elrondPred_of_yojson x = elrondPred_of_yojson (to_list x)
let free_var_to_yojson x = free_var_to_yojson x |> to_assoc
let free_var_of_yojson x = free_var_of_yojson (to_list x)
let elrondTpedvar_to_yojson x = elrondTpedvar_to_yojson x |> to_assoc
let elrondTpedvar_of_yojson x = elrondTpedvar_of_yojson (to_list x)

let elrondForallFormula_to_yojson x =
  elrondForallFormula_to_yojson x |> to_assoc

let elrondForallFormula_of_yojson x = elrondForallFormula_of_yojson (to_list x)
let elrondSpec_to_yojson x = elrondSpec_to_yojson x |> to_assoc
let elrondSpec_of_yojson x = elrondSpec_of_yojson (to_list x)

(* todo maybe no immediate conversions here because of list of string pairs? *)
(* Or we could explicitly tag these pairs as enum pairs *)
let elrondResult_to_yojson x = elrondResult_to_yojson x |> to_assoc
let elrondResult_of_yojson x = elrondResult_of_yojson (to_list x)

let into_elrondValue (v : Pred.Value.t) : elrondValue =
  match v with
  | L l -> L l
  | I i -> I i
  | B b -> B b
  | NotADt -> NotADt
  | _ -> failwith "into_elrondValue: not implemented"

let into_str_val_list (m : Pred.Value.t Utils.StrMap.t) :
    (elrondValue * string) list =
  Utils.StrMap.fold (fun k v acc -> (into_elrondValue v, k) :: acc) m []

let into_elrondType (t : Tp.Tp.t) : elrondType =
  match t with
  | Tp.Tp.Bool -> Bool
  | Tp.Tp.Int -> Int
  | Tp.Tp.IntList -> IntList
  | Tp.Tp.IntTree -> IntTree
  | Tp.Tp.IntTreeI -> IntTreeI
  | Tp.Tp.IntTreeB -> IntTreeB

let into_elrondTpedvar (t : Tp.Tp.tpedvar) : elrondTpedvar =
  let tp, v = t in
  (into_elrondType tp, v)

let into_elrondLit (l : Inference.SingleAbd.SE.L.t) : elrondLiteral =
  match l with
  | Int i -> Int i
  | Bool b -> Bool b
  | IntList l -> IntList l
  | IntTree _t -> (* IntTree t *) failwith "into_elrondLit: not implemented"

let rec into_elrondSimpleExpr (e : Inference.SingleAbd.SE.t) : elrondSimpleExpr
    =
  match e with
  | Literal (t, l) -> Literal (into_elrondType t, into_elrondLit l)
  | Var (t, v) -> Var (into_elrondType t, v)
  | Op (t, op, es) ->
      Op (into_elrondType t, op, List.map into_elrondSimpleExpr es)

let rec into_elrondPred (p : Language.SpecAst.E.t) : elrondPred =
  match p with
  | True -> True
  | Atom e -> Atom (into_elrondSimpleExpr e)
  | Implies (p1, p2) -> Implies (into_elrondPred p1, into_elrondPred p2)
  | Ite (p1, p2, p3) ->
      Ite (into_elrondPred p1, into_elrondPred p2, into_elrondPred p3)
  | Not p -> Not (into_elrondPred p)
  | And ps -> And (List.map into_elrondPred ps)
  | Or ps -> Or (List.map into_elrondPred ps)
  | Iff (p1, p2) -> Iff (into_elrondPred p1, into_elrondPred p2)

let into_elrondFreeVar (fv : Language.Epr.free_variable) : free_var =
  let t, v = fv in
  (into_elrondType t, v)

let into_elrondForallFormula (f : Language.SpecAst.E.forallformula) :
    elrondForallFormula =
  let fv, p = f in
  (List.map into_elrondFreeVar fv, into_elrondPred p)

let into_elrondSpec (s : Language.SpecAst.spec) : elrondSpec =
  let fv, f = s in
  (List.map into_elrondTpedvar fv, into_elrondForallFormula f)

let into_elrondResult (mii : Inference.SpecAbduction.multi_infer_result) :
    elrondResult =
  match mii with
  | Cex cex -> Cex (List.map into_str_val_list cex)
  | Result res ->
      Result
        (Utils.StrMap.fold
           (fun k v acc -> (into_elrondSpec v, k) :: acc)
           res [])

let into_elrondMessage (elrondResult : elrondResult) : elrondMessage =
  Result elrondResult
