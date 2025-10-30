let () =
  let sign, verif = Crux.key_gen () in
  let payload = "this is a test payload" in
  let signature = Crux.sign ~key:sign payload in
  assert (Crux.verify ~signature ~key:verif payload);
  let sign2, verif2 = Crux.key_gen () in
  let signature2 = Crux.sign ~key:sign2 payload in
  assert (Crux.verify ~signature:signature2 ~key:verif2 payload);
  assert (not @@ Crux.verify ~signature ~key:verif2 payload);
  assert (not @@ Crux.verify ~signature:signature2 ~key:verif payload);
  assert (not @@ Crux.verify ~signature ~key:verif "another payload");
  Printf.printf "all tests complete!\n"
