
pub enum ProjectType {
    App,
    Addon,
    Engine,
}

pub enum Feature {
    ApplicationTemplateWrapper(bool),
    TemplateOnlyGlimmerComponents(bool),
}

pub struct AddonConfig {
    version: str,
}

pub struct Preset {
    blueprint: String,
    project: ProjectType,
    features: Vec<Feature>,
    welcome: bool,
    // addons: Map<String, AddonConfig>;
}

pub enum Presets {
    Beginner,
    EmberApp,
    EmberAddon,
    OctaneApp,
    OctaneAddon
}

impl Preset {
    pub fn new(preset: &Presets) -> Preset {
        match preset {
            Beginner => Preset {
                blueprint: String::from(""),
                project: ProjectType::App,
                features: vec![Feature::ApplicationTemplateWrapper(false), Feature::TemplateOnlyGlimmerComponents(true)],
                welcome: false,
                // addons: Map<String, AddonConfig>;
            },
            _ => Preset {
                blueprint: String::from(""),
                project: ProjectType::App,
                features: vec![],
                welcome: false,
            },
        }
    }
}
