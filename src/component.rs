use fs_extra::file::write_all;
use super::utils::{update_values_in_files, uppercase_first_letter};

pub fn generate_component(name: &str, component_class: bool) {
  // TODO: if app/components does not exist throw error
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

  generate_component_test(name);

}

pub fn generate_component_class(name: &str) {
  let path = &format!("app/components/{}.js", name);
  write_all(
    path,
    "import Component from '@glimmer/component';

export default class PLACEHOLDER extends Component {
}"
  );

  update_values_in_files("PLACEHOLDER", &format!("{}Component", uppercase_first_letter(name)), path);

  println!("Created app/components/{}.js", name);

}

pub fn generate_component_test(name: &str) {
  let path: String = format!("tests/integration/components/{}-test.js", name);
  // check if tests/integration

  write_all(
    &path,
    "import { module, test } from 'qunit';
import { setupRenderingTest } from 'ember-qunit';
import { render } from '@ember/test-helpers';
import { hbs } from 'ember-cli-htmlbars';

module('Integration | Component | KEBAB_NAME', function(hooks) {
  setupRenderingTest(hooks);

  test('it renders', async function(assert) {
    // Set any properties with this.set('myProperty', 'value');
    // Handle any actions with this.set('myAction', function(val) { ... });

    await render(hbs`<PASCAL_NAME />`);

    assert.equal(this.element.textContent.trim(), '');

    // Template block usage:
    await render(hbs`
      <PASCAL_NAME>
        template block text
      </PASCAL_NAME>
    `);

    assert.equal(this.element.textContent.trim(), 'template block text');
  });
});
"); 

  update_values_in_files("PASCAL_NAME", &format!("{}", uppercase_first_letter(name)), &path);
  update_values_in_files("KEBAB_NAME", &format!("{}", name), &path);

  println!("Created tests/integration/components/{}-test.js", name);
}