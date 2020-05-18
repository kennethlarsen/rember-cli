pub fn get_editor_config() -> &'static str {
  return r#"
  # EditorConfig helps developers define and maintain consistent
  # coding styles between different editors and IDEs
  # editorconfig.org
  
  root = true
  
  
  [*]
  end_of_line = lf
  charset = utf-8
  trim_trailing_whitespace = true
  insert_final_newline = true
  indent_style = space
  indent_size = 2
  
  [*.hbs]
  insert_final_newline = false
  
  [*.{diff,md}]
  trim_trailing_whitespace = false"#;
}

pub fn get_ember_cli() -> &'static str {
  return r#"
  {
      /**
           Ember CLI sends analytics information by default. The data is completely
          anonymous, but there are times when you might want to disable this behavior.
      
          Setting `disableAnalytics` to true will prevent any data from being sent.
      */
      "disableAnalytics": false
      }"#;
}

pub fn get_eslint_ignore() -> &'static str {
  return r#"
  # unconventional js
  /blueprints/*/files/
  /vendor/
  
  # compiled output
  /dist/
  /tmp/
  
  # dependencies
  /bower_components/
  /node_modules/
  
  # misc
  /coverage/
  !.*
  
  # ember-try
  /.node_modules.ember-try/
  /bower.json.ember-try
  /package.json.ember-try"#;
}

pub fn get_eslintrc_js() -> &'static str {
  return r#"
  module.exports = {
      root: true,
      parserOptions: {
          ecmaVersion: 2017,
          sourceType: 'module'
      },
      plugins: [
          'ember'
      ],
      extends: [
          'eslint:recommended',
          'plugin:ember/recommended'
      ],
      env: {
          browser: true
      },
      rules: {
      },
      overrides: [
          // node files
          {
          files: [
              '.eslintrc.js',
              '.template-lintrc.js',
              'ember-cli-build.js',<% if (blueprint !== 'app') { %>
              'index.js',<% } %>
              'testem.js',
              'blueprints/*/index.js',
              'config/**/*.js'<% if (blueprint === 'app') { %>,
              'lib/*/index.js'<% } %><% if (blueprint !== 'app') { %>,
              'tests/dummy/config/**/*.js'<% } %>
          ],<% if (blueprint !== 'app') { %>
          excludedFiles: [
              'addon/**',
              'addon-test-support/**',
              'app/**',
              'tests/dummy/app/**'
          ],<% } %>
          parserOptions: {
              sourceType: 'script',
              ecmaVersion: 2015
          },
          env: {
              browser: false,
              node: true
          }<% if (blueprint !== 'app') { %>,
          plugins: ['node'],
          rules: Object.assign({}, require('eslint-plugin-node').configs.recommended.rules, {
              // add your custom rules and overrides for node files here
          })<% } %>
          }
      ]
      };"#;
}

pub fn get_template_lintrc_js() -> &'static str {
  return r#"
  'use strict';
  
  module.exports = {
      extends: 'recommended'
  };"#;
}

pub fn get_watchmanconfig() -> &'static str {
  return r#"
  {
      "ignore_dirs": ["tmp", "dist"]
  }"#;
}

pub fn get_ember_cli_build() -> &'static str {
  return r#"
  'use strict';
  
  const EmberApp = require('ember-cli/lib/broccoli/ember-app');
  
  module.exports = function(defaults) {
      let app = new EmberApp(defaults, {
      // Add options here
      });
  
      // Use `app.import` to add additional libraries to the generated
      // output files.
      //
      // If you need to use different assets in different
      // environments, specify an object as the first parameter. That
      // object's keys should be the environment name and the values
      // should be the asset to use in that environment.
      //
      // If the library that you are including contains AMD or ES6
      // modules that you would like to import into your application
      // please specify an object with the list of modules as keys
      // along with the exports of each module as its value.
  
      return app.toTree();
  };"#;
}

pub fn get_gitignore() -> &'static str {
  return r#"
  # See https://help.github.com/ignore-files/ for more about ignoring files.
  
  # compiled output
  /dist/
  /tmp/
  
  # dependencies
  /bower_components/
  /node_modules/
  
  # misc
  /.env*
  /.pnp*
  /.sass-cache
  /connect.lock
  /coverage/
  /libpeerconnection.log
  /npm-debug.log*
  /testem.log
  /yarn-error.log
  
  # ember-try
  /.node_modules.ember-try/
  /bower.json.ember-try
  /package.json.ember-try"#;
}

pub fn get_readme_md() -> &'static str {
  return r#"
  # <%= name %>
  
  This README outlines the details of collaborating on this Ember application.
  A short introduction of this app could easily go here.
  
  ## Prerequisites
  
  You will need the following things properly installed on your computer.
  
  * [Git](https://git-scm.com/)
  * [Node.js](https://nodejs.org/)<% if (yarn) { %>
  * [Yarn](https://yarnpkg.com/)<% } else { %> (with npm)<% } %>
  * [Ember CLI](https://ember-cli.com/)
  * [Google Chrome](https://google.com/chrome/)
  
  ## Installation
  
  * `git clone <repository-url>` this repository
  * `cd <%= name %>`
  * `<% if (yarn) { %>yarn<% } else { %>npm<% } %> install`
  
  ## Running / Development
  
  * `ember serve`
  * Visit your app at [http://localhost:4200](http://localhost:4200).
  * Visit your tests at [http://localhost:4200/tests](http://localhost:4200/tests).
  
  ### Code Generators
  
  Make use of the many generators for code, try `ember help generate` for more details
  
  ### Running Tests
  
  * `ember test`
  * `ember test --server`
  
  ### Linting
  
  * `<% if (yarn) { %>yarn lint:hbs<% } else { %>npm run lint:hbs<% } %>`
  * `<% if (yarn) { %>yarn lint:js<% } else { %>npm run lint:js<% } %>`
  * `<% if (yarn) { %>yarn lint:js --fix<% } else { %>npm run lint:js -- --fix<% } %>`
  
  ### Building
  
  * `ember build` (development)
  * `ember build --environment production` (production)
  
  ### Deploying
  
  Specify what it takes to deploy your app.
  
  ## Further Reading / Useful Links
  
  * [ember.js](https://emberjs.com/)
  * [ember-cli](https://ember-cli.com/)
  * Development Browser Extensions
  * [ember inspector for chrome](https://chrome.google.com/webstore/detail/ember-inspector/bmdblncegkenkacieihfhpjfppoconhi)
  * [ember inspector for firefox](https://addons.mozilla.org/en-US/firefox/addon/ember-inspector/)"#;
}
