pub fn get_app_js() -> &'static str {
  return r#"
import Application from '@ember/application';
import Resolver from './resolver';
import loadInitializers from 'ember-load-initializers';
import config from './config/environment';

const App = Application.extend({
modulePrefix: config.modulePrefix,
podModulePrefix: config.podModulePrefix,
Resolver
});

loadInitializers(App, config.modulePrefix);

export default App;"#;
}

pub fn get_resolver_js() -> &'static str {
  return r#"
import Resolver from 'ember-resolver';

export default Resolver;"#;
}

pub fn get_router_js() -> &'static str {
  return r#"
import EmberRouter from '@ember/routing/router';
import config from './config/environment';

const Router = EmberRouter.extend({
    location: config.locationType,
    rootURL: config.rootURL
});

Router.map(function() {
});

export default Router;"#;
}

pub fn get_environment_js() -> &'static str {
  return r#"
'use strict';

module.exports = function(environment) {
    let ENV = {
    modulePrefix: '<%= modulePrefix %>',
    environment,
    rootURL: '/',
    locationType: 'auto',
    EmberENV: {
        FEATURES: {
        // Here you can enable experimental features on an ember canary build
        // e.g. EMBER_NATIVE_DECORATOR_SUPPORT: true
        },
        EXTEND_PROTOTYPES: {
        // Prevent Ember Data from overriding Date.parse.
        Date: false
        }
    },

    APP: {
        // Here you can pass flags/options to your application instance
        // when it is created
    }
    };

    if (environment === 'development') {
    // ENV.APP.LOG_RESOLVER = true;
    // ENV.APP.LOG_ACTIVE_GENERATION = true;
    // ENV.APP.LOG_TRANSITIONS = true;
    // ENV.APP.LOG_TRANSITIONS_INTERNAL = true;
    // ENV.APP.LOG_VIEW_LOOKUPS = true;
    }

    if (environment === 'test') {
    // Testem prefers this...
    ENV.locationType = 'none';

    // keep test console output quieter
    ENV.APP.LOG_ACTIVE_GENERATION = false;
    ENV.APP.LOG_VIEW_LOOKUPS = false;

    ENV.APP.rootElement = '#ember-testing';
    ENV.APP.autoboot = false;
    }

    if (environment === 'production') {
    // here you can enable a production-specific feature
    }

    return ENV;
};"#;
}

pub fn get_target_js() -> &'static str {
  return r#"
'use strict';
  
const browsers = [
    'last 1 Chrome versions',
    'last 1 Firefox versions',
    'last 1 Safari versions'
];

const isCI = !!process.env.CI;
const isProduction = process.env.EMBER_ENV === 'production';

if (isCI || isProduction) {
    browsers.push('ie 11');
}

module.exports = {
    browsers
};"#;
}

pub fn get_test_helper_js() -> &'static str {
  return r#"
  import Application from '../app';
  import config from '../config/environment';
  import { setApplication } from '@ember/test-helpers';
  import { start } from 'ember-qunit';
  
  setApplication(Application.create(config.APP));
  
  start();"#;
}

pub fn get_testem_js() -> &'static str {
  return r#"
  module.exports = {
      test_page: 'tests/index.html?hidepassed',
      disable_watching: true,
      launch_in_ci: [
          'Chrome'
      ],
      launch_in_dev: [
          'Chrome'
      ],
      browser_args: {
          Chrome: {
          ci: [
              // --no-sandbox is needed when running Chrome inside a container
              process.env.CI ? '--no-sandbox' : null,
              '--headless',
              '--disable-gpu',
              '--disable-dev-shm-usage',
              '--disable-software-rasterizer',
              '--mute-audio',
              '--remote-debugging-port=0',
              '--window-size=1440,900'
          ].filter(Boolean)
          }
      }
      };"#;
}
