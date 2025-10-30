let key_gen () =
  let sign_key = Bytes.create 4032 in
  let verif_key = Bytes.create 1952 in
  Rust.key_gen sign_key verif_key;
  (Bytes.unsafe_to_string sign_key, Bytes.unsafe_to_string verif_key)

let sign ~key payload =
  let signature = Bytes.create 3309 in
  let ok =
    Rust.sign
      (Bytes.unsafe_of_string payload)
      (Bytes.unsafe_of_string key)
      signature
  in
  if ok then Bytes.unsafe_to_string signature else invalid_arg "libcrux failed"

let verify ~signature ~key payload =
  Rust.verify
    (Bytes.unsafe_of_string payload)
    (Bytes.unsafe_of_string signature)
    (Bytes.unsafe_of_string key)
