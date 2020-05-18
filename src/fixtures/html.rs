pub fn get_test_index_html() -> &'static str {
  return r#"
  <!DOCTYPE html>
  <html>
      <head>
      <meta charset="utf-8">
      <meta http-equiv="X-UA-Compatible" content="IE=edge">
      <title><%= namespace %> Tests</title>
      <meta name="description" content="">
      <meta name="viewport" content="width=device-width, initial-scale=1">
  
      {{content-for "head"}}
      {{content-for "test-head"}}
  
      <link rel="stylesheet" href="{{rootURL}}assets/vendor.css">
      <link rel="stylesheet" href="{{rootURL}}assets/<%= name %>.css">
      <link rel="stylesheet" href="{{rootURL}}assets/test-support.css">
  
      {{content-for "head-footer"}}
      {{content-for "test-head-footer"}}
      </head>
      <body>
      {{content-for "body"}}
      {{content-for "test-body"}}
  
      <script src="/testem.js" integrity=""></script>
      <script src="{{rootURL}}assets/vendor.js"></script>
      <script src="{{rootURL}}assets/test-support.js"></script>
      <script src="{{rootURL}}assets/<%= name %>.js"></script>
      <script src="{{rootURL}}assets/tests.js"></script>
  
      {{content-for "body-footer"}}
      {{content-for "test-body-footer"}}
      </body>
  </html>"#;
}
