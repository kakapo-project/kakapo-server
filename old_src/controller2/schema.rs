
use objekt::Clone;

use super::types::{DataType, DataPoint};


#[derive(Deserialize, Serialize, Clone)]
pub enum Reference {
    Table(String),
    TableOnColumn {
        table: String,
        from_column: String,
    },
    TableWithAnotherColumn {
        table: String,
        link: String,
    }
}

impl Reference {
    pub fn table(table_name: &str) -> Self {
        Reference::Table(table_name.to_owned())
    }

    pub fn table_on_column(table_name: &str, from_column: &str) -> Self {
        Reference::TableOnColumn {
            table: table_name.to_owned(),
            from_column: from_column.to_owned(),
        }
    }

    pub fn table_with_another_column(table_name: &str, from_column: &str) -> Self {
        Reference::TableWithAnotherColumn {
            table: table_name.to_owned(),
            link: from_column.to_owned(),
        }
    }
}


#[derive(Deserialize, Serialize, Clone)]
pub enum Constraint {
    Unique { column_name: String },
    PrimaryKey { column_name: String },
    ForeignKeyDefault { table: String, column_name: String },
    ForeignKey { table: String, links: Vec<(String, String)>},
    OneOf { column_name: String, accepted_values: Vec<DataPoint> },
    Equals { column_name: String, accepted_value: DataPoint },

}

impl Constraint {
    pub fn unique(column_name: &str) -> Self {
        Constraint::Unique {
            column_name:  column_name.to_owned(),
        }
    }

    pub fn primary_key(column_name: &str) -> Self {
        Constraint::PrimaryKey {
            column_name:  column_name.to_owned(),
        }
    }

    pub fn foreign_key_default(table: &str, column_name: &str) -> Self {
        Constraint::ForeignKeyDefault {
            table: table.to_owned(),
            column_name: column_name.to_owned(),
        }
    }

    pub fn foreign_key(table: &str, links: &Vec<(&str, &str)>) -> Self {

        let link_ids = links
            .iter()
            .map(|&x| {
                let (x1, x2) = x;
                (x1.to_owned(), x2.to_owned())
            })
            .collect::<Vec<_>>();

        Constraint::ForeignKey {
            table: table.to_owned(),
            links: link_ids
        }
    }

    pub fn one_of(column_name: &str, accepted_values: &Vec<DataPoint> ) -> Self {
        Constraint::OneOf {
            column_name:  column_name.to_owned(),
            accepted_values: accepted_values.to_owned(),
        }
    }

    pub fn equals(column_name: &str, accepted_value: &DataPoint ) -> Self {
        Constraint::Equals {
            column_name:  column_name.to_owned(),
            accepted_value: accepted_value.to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Column {
    name: String,
    data_type: DataType,
}

impl Column {
    pub fn new(name: &str, data_type: &DataType) -> Self {
        Column {
            name: name.to_owned(),
            data_type: data_type.to_owned()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}


/// Schema
#[derive(Deserialize, Serialize, Clone)]
pub struct Schema {
    table_id: String,
    columns: Vec<Column>,
    constraints: Vec<Constraint>,
}

impl Schema {
    pub fn new(table_name: &str) -> Self {
        Schema {
            table_id: table_name.to_owned(),
            columns: vec![],
            constraints: vec![],
        }
    }

    pub fn get_name(&self) -> String {
        let Schema { table_id, .. } = self;
        table_id.to_owned()
    }

    pub fn get_columns(&self) -> Vec<Column> {
        let Schema { columns, .. } = self;
        columns.to_owned()
    }

    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        let column_info = self.get_columns();
        let column_index = column_info
            .iter()
            .position(|column| column.get_name() == column_name.to_owned());
        column_index
    }

    pub fn inherited_by(&self, children: &Vec<&str>) -> Self {

        let accepted_values = children
            .iter()
            .map(|&x|
                DataPoint::String(x.to_owned())
            )
            .collect::<Vec<_>>();

        let table_name = self.table_id.to_owned();
        let type_name = format!("{}_type", table_name);
        let type_name_str = &type_name[..];
        self.column(type_name_str, &DataType::StringType)
            .constraint(&Constraint::one_of(type_name_str, &accepted_values))
    }

    pub fn inherits(&self, parent: &str) -> Self {
        let table_name = self.table_id.to_owned();
        let table_name_str = &table_name[..];

        let type_name = format!("{}_type", parent);
        let type_name_str = &type_name[..];

        self.reference(&Reference::table_with_another_column(parent, type_name_str))
            .constraint(&Constraint::equals(type_name_str, &DataPoint::String(table_name_str.to_owned())))

    }

    pub fn junction(&self, table_a: &str, table_b: &str) -> Self {

        self.reference(&Reference::table(table_a))
            .reference(&Reference::table(table_b))

    }

    pub fn id_column(&self) -> Self {
        let table_name = self.table_id.to_owned();
        let id_name = format!("{}_id", table_name);
        let id_name_str = &id_name[..];
        self.column(id_name_str, &DataType::SerialType)
            .constraint(&Constraint::primary_key(id_name_str))
    }

    pub fn reference(&self, reference: &Reference) -> Self {

        match reference {
            Reference::Table(table_id) => {
                let table_name = table_id.to_owned();
                let column_name = format!("{}_id", table_name);

                self.column(&column_name, &DataType::IntegerType)
                    .constraint(&Constraint::foreign_key_default(&table_name[..], &column_name[..]))
            },
            Reference::TableOnColumn { table, from_column } => {
                let table_name = table.to_owned();
                let column_name = from_column.to_owned();
                self.column(&column_name, &DataType::IntegerType)
                    .constraint(&Constraint::foreign_key_default(&table_name[..], &column_name[..]))
            },
            Reference::TableWithAnotherColumn { table, link } => {
                let table_name = table.to_owned();

                let column_a_name = format!("{}_id", table_name); //TODO: check if table_id is valid
                let column_b_name = link.to_owned();

                self.column(&column_a_name, &DataType::IntegerType)
                    .column(&column_b_name, &DataType::IntegerType)
                    .constraint(&Constraint::foreign_key(&table_name, &vec![
                        (&column_a_name[..], &column_a_name[..]),
                        (&column_b_name[..], &column_b_name[..]),
                    ]))
            },
        }
    }

    pub fn column(&self, column_name: &str, data_type:&DataType) -> Self {
        //TODO: check if not duplicate column
        let column = Column::new(column_name, data_type);

        let Schema { columns, .. } = self;

        let mut new_columns = columns.to_vec();
        new_columns.push(column);

        Schema {
            columns: new_columns,
            ..self.to_owned()
        }
    }

    pub fn constraint(&self, constraint: &Constraint) -> Self {
        let Schema { constraints, .. } = self;

        let mut new_constraints = constraints.to_vec();
        new_constraints.push(constraint.to_owned());

        Schema {
            constraints: new_constraints,
            ..self.to_owned()
        }
    }
}
