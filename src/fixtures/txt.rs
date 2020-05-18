pub fn get_robots_txt() -> &'static str {
  return r#"
# http://www.robotstxt.org
User-agent: *
Disallow:"#;
}
