pub fn main() -> std::io::Result<()> {
  ocaml_build::Sigs::new("rust.ml").generate()
}
