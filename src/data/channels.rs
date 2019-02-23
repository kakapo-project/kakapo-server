use model::entity::RawEntityTypes;
use data::auth::User;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Defaults {
    Table(String),
    Query(String),
    Script(String),
    //TODO: view
    TableData(String), //TODO: this is tricky since the filter / query can go in as well
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channels {
    Defaults(Defaults),
    Subscribers(Defaults),
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
