let usage () =
  Printf.printf "cli.exe kp\n";
  Printf.printf "cli.exe sign PAYLOAD_FILE KEY_FILE\n"

let key_gen () =
  let sign, verif = Crux.key_gen () in
  Out_channel.(with_open_bin "sign.key" (fun ch -> output_string ch sign));
  Out_channel.(with_open_bin "verif.key" (fun ch -> output_string ch verif));
  Printf.printf "OK (sign.key, verif.key)\n"

let sign in_file key =
  let key = In_channel.(with_open_bin key input_all) in
  let payload = In_channel.(with_open_bin in_file input_all) in
  let signature = Crux.sign ~key payload in
  Out_channel.(
    with_open_bin (in_file ^ ".signed") (fun ch ->
        output_string ch signature;
        output_string ch payload))

let () =
  match Sys.argv |> Array.to_list with
  | _ :: "kp" :: _ -> key_gen ()
  | _ :: "sign" :: in_file :: key :: _ -> sign in_file key
  | _ -> usage ()
