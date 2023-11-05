use texformat::texformat;

fn main() {
  let name = "Alice";
  let formatted = texformat!("Hello, $(name)!");
  assert_eq!("Hello, Alice!", formatted);
  println!("{}", formatted);
}
