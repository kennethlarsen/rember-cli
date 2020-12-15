pub fn get_optional_features() -> &'static str {
  return r#"
{
  "application-template-wrapper": false,
  "default-async-observers": true,
  "jquery-integration": false,
  "template-only-glimmer-components": true
}"#;
}

pub fn get_package_json() -> &'static str {
  return r#"
  {
      "name": "<%= name %>",
      "version": "0.0.0",
      "private": true,
      "description": "Small description for <%= name %> goes here",
      "repository": "",
      "license": "MIT",
      "author": "",
      "directories": {
        "doc": "doc",
        "test": "tests"
      },
      "scripts": {
        "build": "ember build --environment=production",
        "lint": "npm-run-all --aggregate-output --continue-on-error --parallel lint:*",
        "lint:hbs": "ember-template-lint .",
        "lint:js": "eslint .",
        "start": "ember serve",
        "test": "npm-run-all lint:* test:*",
        "test:ember": "ember test"
      },
      "devDependencies": {
        "@ember/optional-features": "^2.0.0",
        "@glimmer/component": "^1.0.2",
        "@glimmer/tracking": "^1.0.2",
        "babel-eslint": "^10.1.0",
        "broccoli-asset-rev": "^3.0.0",
        "ember-auto-import": "^1.6.0",
        "ember-cli": "~3.22.0",
        "ember-cli-app-version": "^4.0.0",
        "ember-cli-babel": "^7.22.1",
        "ember-cli-dependency-checker": "^3.2.0",
        "ember-cli-htmlbars": "^5.3.1",
        "ember-cli-inject-live-reload": "^2.0.2",
        "ember-cli-sri": "^2.1.1",
        "ember-cli-terser": "^4.0.0",
        "ember-data": "~3.22.0",
        "ember-export-application-global": "^2.0.1",
        "ember-fetch": "^8.0.2",
        "ember-load-initializers": "^2.1.1",
        "ember-maybe-import-regenerator": "^0.1.6",
        "ember-qunit": "^4.6.0",
        "ember-resolver": "^8.0.2",
        "ember-source": "~3.22.0",
        "ember-template-lint": "^2.14.0",
        "ember-welcome-page": "^4.0.0",
        "eslint": "^7.11.0",
        "eslint-plugin-ember": "^9.3.0",
        "eslint-plugin-node": "^11.1.0",
        "loader.js": "^4.7.0",
        "npm-run-all": "^4.1.5",
        "qunit-dom": "^1.5.0"
      },
      "engines": {
        "node": "10.* || >= 12"
      },
      "ember": {
        "edition": "octane"
      }
  }"#;
}
