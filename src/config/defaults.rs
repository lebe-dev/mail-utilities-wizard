#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DefaultsConfig {
    pub template: NonBlankString,
    pub signature: NonBlankString
}