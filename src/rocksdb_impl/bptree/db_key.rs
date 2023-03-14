#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DbKey {
    data: [u8; DbKey::LenDbKey],
}

impl From<&[u8]> for DbKey {
    fn from(value: &[u8]) -> Self {
        if value.len() > DbKey::LenDbKey {
            DbKey { data: value[0..DbKey::LenDbKey].try_into().expect("") }
        } else if value.len() == DbKey::LenDbKey {
            DbKey { data: value.clone().try_into().expect("") }
        } else {
            panic!("the db key is less 12")
        }
    }
}

impl DbKey {
    pub const LenDbKey: usize = 12;
    pub const ZeroKey: [u8; DbKey::LenDbKey] = [0; DbKey::LenDbKey];
    pub fn key(&self) -> &[u8] {
        &self.data
    }
}
