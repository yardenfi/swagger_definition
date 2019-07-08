use crate::Ref;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ValueOrRef<T> {
    Ref(Ref),
    Value(T)
}
