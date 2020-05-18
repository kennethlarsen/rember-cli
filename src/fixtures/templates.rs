pub fn get_application_hbs() -> &'static str {
  return r#"
{{!-- The following component displays Ember's default welcome message. --}}
  <WelcomePage />
  {{!-- Feel free to remove this! --}}
{{outlet}}"#;
}

pub fn get_index_html() -> &'static str {
  return r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <title><%= namespace %></title>
    <meta name="description" content="">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    {{content-for "head"}}

    <link integrity="" rel="stylesheet" href="{{rootURL}}assets/vendor.css">
    <link integrity="" rel="stylesheet" href="{{rootURL}}assets/<%= name %>.css">

    {{content-for "head-footer"}}
  </head>
  <body>
    {{content-for "body"}}

    <script src="{{rootURL}}assets/vendor.js"></script>
    <script src="{{rootURL}}assets/<%= name %>.js"></script>

    {{content-for "body-footer"}}
  </body>
</html>"#;
}
