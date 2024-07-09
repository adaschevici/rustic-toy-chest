pub trait ToJson {
    fn to_json(&self) -> String;
}

pub trait ToJsonGeneric {
    fn to_json(&self) -> String;
}
