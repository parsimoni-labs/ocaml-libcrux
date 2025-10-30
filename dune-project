(lang dune 3.0)

(generate_opam_files true)
(maintainers "Franziskus Kiefer" "Virgile Robles")
(name crux)
(version 0.1)

(package
  (name crux)
  (synopsis "Post-quantum DSA functions with libcrux")
  (depends
    (conf-rust :build)))
