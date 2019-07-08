#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ref {
    #[serde(rename = "$ref")]
    pub ref_: String
}

#[cfg(test)]
mod tests {
    use crate::Ref;

    #[test]
    fn create_ref() {
        Ref {
            ref_: "thing".to_owned()
        };
    }
}