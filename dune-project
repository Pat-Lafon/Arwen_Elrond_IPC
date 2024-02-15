(lang dune 3.8)
(name my_project)

(generate_opam_files true)

(package
 (name my_project)
 (synopsis "FIXME:")
 (description "FIXME:")
 (depends
  (ocaml
   (>= "4.03.0"))
  (dune
   (>= "1.5"))
  ADT-Lemma-Discovery
  conf-rust))