#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    pub name: NonBlankString,
    pub email: Email,
    pub template: String,
    pub signature: String,
}