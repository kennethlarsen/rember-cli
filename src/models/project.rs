pub struct ProjectSetupConfig {
    npm_client: String;
    blueprint: String;
    skip_git: bool;
    skip_npm: bool;
    directory: String;
}

pub struct Project {
    npm_client: String;
    directory: String;
}
