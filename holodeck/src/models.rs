use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    id: u64,
    item: Item,
}

impl Todo {
    pub fn id(&self) -> u64 {
        self.id
    }
}

/// 名前を表す型の定義
#[derive(Clone, Debug, Serialize)]
struct Item(String);

impl Item {
    /// 値のチェックを行った上でNameを作成する
    pub fn new(item: &str) -> Result<Self, String> {
        let size = item.chars().count();
        if size < 1 || size > 10 {
            return Err("名前は10文字以内です".to_string());
        }

        if item.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err("名前が使用できる文字種はA-Z, a-zです".to_string());
        }
        Ok(Item(item.to_string()))
    }
}

impl<'de> de::Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Item::new(&s).map_err(de::Error::custom)
    }
}
/// 文字列からの変換を表す
/// このtraitの実装をwarp::path::params()関数が要求する
impl std::str::FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Item::new(s)
    }
}

/// handlerでformatを行うために要求される
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_name() {
    let ok_value = "Nrskt";
    assert!(Item::new(ok_value).is_ok());

    let ok_value = "N";
    assert!(Item::new(ok_value).is_ok());

    let ok_value = "NrsktNrskt";
    assert!(Item::new(ok_value).is_ok());

    let ng_value = "0";
    assert!(Item::new(ng_value).is_err());

    let ng_value = "";
    assert!(Item::new(ng_value).is_err());

    let ng_value = "NrsktNrsktN";
    assert!(Item::new(ng_value).is_err());
}
