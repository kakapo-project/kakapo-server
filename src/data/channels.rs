use model::entity::RawEntityTypes;
use data::auth::User;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Defaults {
    Table(String),
    Query(String),
    Script(String),
    //TODO: view
    TableData(String), //TODO: this is tricky since the filter / query can go in as well
}

//A little bit messy as there isn't currently a way in serde to organize this
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Sub {
    Subscribers(Defaults),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Channels {
    Defaults(Defaults),
    Subscribers(Sub),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub user: User,
    pub channel: Channels,
}

impl Channels {

    pub fn entity<T>(name: &str) -> Self
        where T: RawEntityTypes,
    {
        Channels::Defaults(Defaults::Table(name.to_string()))
    }

    pub fn table(table_name: &str) -> Self {
        Channels::Defaults(Defaults::TableData(table_name.to_string()))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_channel() {
        let channel = Channels::Defaults(Defaults::Table("test".to_string()));

        let repr = serde_json::to_value(&channel).unwrap();
        assert_eq!(repr, json!({"table": "test"}));

        let channel = Channels::Subscribers(Sub::Subscribers(Defaults::Table("test".to_string())));

        let repr = serde_json::to_value(&channel).unwrap();
        assert_eq!(repr, json!({"type": "subscribers", "table": "test"}));
    }
}