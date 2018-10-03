
use objekt::Clone;

use super::types::{DataType, DataPoint, Identifier};


#[derive(Deserialize, Serialize, Clone)]
pub enum Reference {
    Table(Identifier),
    TableOnColumn {
        table: Identifier,
        from_column: Identifier,
    },
    TableWithAnotherColumn {
        table: Identifier,
        link: Identifier,
    }
}

impl Reference {
    pub fn table(table_name: &str) -> Self {
        Reference::Table(Identifier::new(table_name))
    }

    pub fn table_on_column(table_name: &str, from_column: &str) -> Self {
        Reference::TableOnColumn {
            table: Identifier::new(table_name),
            from_column: Identifier::new(from_column),
        }
    }

    pub fn table_with_another_column(table_name: &str, from_column: &str) -> Self {
        Reference::TableWithAnotherColumn {
            table: Identifier::new(table_name),
            link: Identifier::new(from_column),
        }
    }
}


#[derive(Deserialize, Serialize, Clone)]
pub enum Constraint {
    Unique { column_name: Identifier },
    PrimaryKey { column_name: Identifier },
    ForeignKeyDefault { table: Identifier, column_name: Identifier },
    ForeignKey { table: Identifier, links: Vec<(Identifier, Identifier)>},
    OneOf { column_name: Identifier, accepted_values: Vec<DataPoint> },
    Equals { column_name: Identifier, accepted_value: DataPoint },

}

impl Constraint {
    pub fn unique(column_name: &str) -> Self {
        Constraint::Unique {
            column_name:  Identifier::new(column_name),
        }
    }

    pub fn primary_key(column_name: &str) -> Self {
        Constraint::PrimaryKey {
            column_name:  Identifier::new(column_name),
        }
    }

    pub fn foreign_key_default(table: &str, column_name: &str) -> Self {
        Constraint::ForeignKeyDefault {
            table: Identifier::new(table),
            column_name: Identifier::new(column_name)
        }
    }

    pub fn foreign_key(table: &str, links: &Vec<(&str, &str)>) -> Self {

        let link_ids = links
            .iter()
            .map(|&x| {
                let (x1, x2) = x;
                (Identifier::new(x1), Identifier::new(x2))
            })
            .collect::<Vec<_>>();

        Constraint::ForeignKey {
            table: Identifier::new(table),
            links: link_ids
        }
    }

    pub fn one_of(column_name: &str, accepted_values: &Vec<DataPoint> ) -> Self {
        Constraint::OneOf {
            column_name:  Identifier::new(column_name),
            accepted_values: accepted_values.to_owned(),
        }
    }

    pub fn equals(column_name: &str, accepted_value: &DataPoint ) -> Self {
        Constraint::Equals {
            column_name:  Identifier::new(column_name),
            accepted_value: accepted_value.to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Column {
    name: Identifier,
    data_type: DataType,
}

impl Column {
    pub fn new(name: &str, data_type: &DataType) -> Self {
        Column {
            name: Identifier::new(name),
            data_type: data_type.to_owned()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_name()
    }
}


/// Schema
#[derive(Deserialize, Serialize, Clone)]
pub struct Schema {
    table_id: Identifier,
    columns: Vec<Column>,
    constraints: Vec<Constraint>,
}

impl Schema {
    pub fn new(table_name: &str) -> Self {
        Schema {
            table_id: Identifier::new(table_name),
            columns: vec![],
            constraints: vec![],
        }
    }

    pub fn get_columns(&self) -> Vec<Column> {
        let Schema { columns, .. } = self;
        columns.to_owned()
    }

    pub fn inherited_by(&self, children: &Vec<&str>) -> Self {

        let accepted_values = children
            .iter()
            .map(|&x|
                DataPoint::String(x.to_owned())
            )
            .collect::<Vec<_>>();

        let table_name = self.table_id.get_name();
        let type_name = format!("{}_type", table_name);
        let type_name_str = &type_name[..];
        self.column(type_name_str, &DataType::StringType)
            .constraint(&Constraint::one_of(type_name_str, &accepted_values))
    }

    pub fn inherits(&self, parent: &str) -> Self {
        let table_name = self.table_id.get_name();
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
        let table_name = self.table_id.get_name();
        let id_name = format!("{}_id", table_name);
        let id_name_str = &id_name[..];
        self.column(id_name_str, &DataType::SerialType)
            .constraint(&Constraint::primary_key(id_name_str))
    }

    pub fn reference(&self, reference: &Reference) -> Self {

        match reference {
            Reference::Table(table_id) => {
                let table_name = table_id.get_name();
                let column_name = format!("{}_id", table_name);

                self.column(&column_name, &DataType::IntegerType)
                    .constraint(&Constraint::foreign_key_default(&table_name[..], &column_name[..]))
            },
            Reference::TableOnColumn { table, from_column } => {
                let table_name = table.get_name();
                let column_name = from_column.get_name();
                self.column(&column_name, &DataType::IntegerType)
                    .constraint(&Constraint::foreign_key_default(&table_name[..], &column_name[..]))
            },
            Reference::TableWithAnotherColumn { table, link } => {
                let table_name = table.get_name();

                let column_a_name = format!("{}_id", table_name); //TODO: check if table_id is valid
                let column_b_name = link.get_name();

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
