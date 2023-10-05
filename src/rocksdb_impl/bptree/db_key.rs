#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DbKey {
    data: [u8; DbKey::LEN_DB_KEY],
}

impl From<&[u8]> for DbKey {
    fn from(value: &[u8]) -> Self {
        if value.len() > DbKey::LEN_DB_KEY {
            DbKey { data: value[0..DbKey::LEN_DB_KEY].try_into().expect("") }
        } else if value.len() == DbKey::LEN_DB_KEY {
            DbKey { data: value.clone().try_into().expect("") }
        } else {
            panic!("the db key is less 12")
        }
    }
}

impl DbKey {
    pub const LEN_DB_KEY: usize = 12;
    pub const ZERO_KEY: [u8; DbKey::LEN_DB_KEY] = [0; DbKey::LEN_DB_KEY];
    pub fn key(&self) -> &[u8] {
        &self.data
    }
}
