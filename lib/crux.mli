val key_gen : unit -> string * string
(** Return a new (sign, verif) keypair *)

val sign : key:string -> string -> string
(** [sign ~key payload] returns a signature of the payload with the given sign
    key *)

val verify : signature:string -> key:string -> string -> bool
(** [verify ~signature ~key payload] where [key] is a verif key returns [true]
    iff [signature] signs [payload] with the matching signining key *)
