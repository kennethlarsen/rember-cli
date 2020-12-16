use rember::utils::{uppercase_first_letter};

#[test]
fn it_uppercases_first_letter() {
  // TODO: Support kebab-case

  let name = "button";
  let uppercase_name = uppercase_first_letter(name);

  assert_eq!(uppercase_name, "Button");
}