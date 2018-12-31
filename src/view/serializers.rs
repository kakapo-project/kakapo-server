
use model::actions::results as actions;

use serde;
use serde_json::Value;
use serde::Serialize;
use data;

pub trait Serializable {
    const ACTION_NAME: &'static str = "NoAction";
    fn into_serialize(self) -> serde_json::Value;
}

impl Serializable for actions::GetAllEntitiesResult<data::Table> {
    const ACTION_NAME: &'static str = "GetAllTables";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::GetAllEntitiesResult<data::Query> {
    const ACTION_NAME: &'static str = "GetAllQueries";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::GetAllEntitiesResult<data::Script> {
    const ACTION_NAME: &'static str = "GetAllScripts";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::GetEntityResult<data::Table> {
    const ACTION_NAME: &'static str = "GetTable";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::GetEntityResult<data::Query> {
    const ACTION_NAME: &'static str = "GetQuery";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::GetEntityResult<data::Script> {
    const ACTION_NAME: &'static str = "GetScript";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::CreateEntityResult<data::Table> {
    const ACTION_NAME: &'static str = "CreateTable";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::CreateEntityResult<data::Query> {
    const ACTION_NAME: &'static str = "CreateQuery";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::CreateEntityResult<data::Script> {
    const ACTION_NAME: &'static str = "CreateScript";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::UpdateEntityResult<data::Table> {
    const ACTION_NAME: &'static str = "UpdateTable";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::UpdateEntityResult<data::Query> {
    const ACTION_NAME: &'static str = "UpdateQuery";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::UpdateEntityResult<data::Script> {
    const ACTION_NAME: &'static str = "UpdateScript";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::DeleteEntityResult<data::Table> {
    const ACTION_NAME: &'static str = "DeleteTable";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::DeleteEntityResult<data::Query> {
    const ACTION_NAME: &'static str = "DeleteQuery";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}


impl Serializable for actions::DeleteEntityResult<data::Script> {
    const ACTION_NAME: &'static str = "DeleteScript";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::GetTableDataResult {
    const ACTION_NAME: &'static str = "GetTableData";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::InsertTableDataResult {
    const ACTION_NAME: &'static str = "InsertTableData";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::UpdateTableDataResult {
    const ACTION_NAME: &'static str = "UpdateTableData";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::DeleteTableDataResult {
    const ACTION_NAME: &'static str = "DeleteTableData";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::RunQueryResult {
    const ACTION_NAME: &'static str = "RunQuery";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for actions::RunScriptResult {
    const ACTION_NAME: &'static str = "RunScript";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}

impl Serializable for () {
    const ACTION_NAME: &'static str = "None";

    fn into_serialize(self) -> Value {
        unimplemented!()
    }
}