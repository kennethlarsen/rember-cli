use fs_extra::file::write_all;
use super::utils::{update_values_in_files, uppercase_first_letter};

pub fn generate_component(name: &str, component_class: bool) {
  // if app/components does not exist throw error
  write_all(
    format!("app/components/{}.hbs", name),
    "{{yield}}"
  );

  println!("Created app/components/{}.hbs", name);

  if component_class {
    generate_component_class(name);
  } else {
    println!("Tip: To create a class run rember generate component-class {}", name);
  }

  println!("Created tests/integration/{}-test.js", name);

}

pub fn generate_component_class(name: &str) {
  let path = &format!("app/components/{}.js", name);
  write_all(
    path,
    "import Component from '@glimmer/component';

export default class PLACEHOLDER extends Component {
}"
  );
  // PascalCase name, spacing in boilerplate
  update_values_in_files("PLACEHOLDER", &format!("{}Component", uppercase_first_letter(name)), path);

  println!("Created app/components/{}.js", name);

}