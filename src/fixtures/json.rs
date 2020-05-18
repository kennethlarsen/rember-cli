pub fn get_optional_features() -> &'static str {
  return r#"
{
    "jquery-integration": true
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
          "build": "ember build",
          "lint:hbs": "ember-template-lint .",
          "lint:js": "eslint .",
          "start": "ember serve",
          "test": "ember test"
      },
      "devDependencies": {
          "@ember/jquery": "^0.6.0",
          "@ember/optional-features": "^0.7.0",
          "broccoli-asset-rev": "^3.0.0",
          "ember-ajax": "^5.0.0",
          "ember-cli": "^3.9.0",
          "ember-cli-app-version": "^3.2.0",
          "ember-cli-babel": "^7.4.1",
          "ember-cli-dependency-checker": "^3.1.0",
          "ember-cli-eslint": "^5.1.0",
          "ember-cli-htmlbars": "^3.0.1",
          "ember-cli-htmlbars-inline-precompile": "^2.1.0",
          "ember-cli-inject-live-reload": "^1.8.2",
          "ember-cli-sri": "^2.1.1",
          "ember-cli-template-lint": "^1.0.0-beta.1",
          "ember-cli-uglify": "^2.1.0",
          "ember-data": "~3.9.0",
          "ember-export-application-global": "^2.0.0",
          "ember-load-initializers": "^2.0.0",
          "ember-maybe-import-regenerator": "^0.1.6",
          "ember-qunit": "^4.4.1",
          "ember-resolver": "^5.0.1",
          "ember-source": "~3.10.0-beta.1",
          "ember-welcome-page": "^4.0.0",
          "eslint-plugin-ember": "^6.2.0",
          "loader.js": "^4.7.0",
          "qunit-dom": "^0.8.4"
      },
      "engines": {
          "node": "6.* || 8.* || >= 10.*"
      }
  }"#;
}
