

use actix::prelude::*;
use diesel::{
    prelude::*,
    insert_into,
    delete,
    update,
};
use failure::Fallible;

use super::schema::user_account::dsl::*;
use super::connection::DatabaseExecutor;

/// The create session message
pub struct CreateTable(pub String);

impl Message for CreateTable {
    type Result = Fallible<UserAccount>;
}

#[derive(Debug)]
pub struct UserAccount {
    username: String,
    password: String,
    email: String,
}

impl Handler<CreateTable> for DatabaseExecutor {
    type Result = Fallible<UserAccount>;

    fn handle(&mut self, msg: CreateTable, _: &mut Self::Context) -> Self::Result {
        // Insert the session into the database
        println!("Creating new user: {}", msg.0);
        /*
        Ok(insert_into(user_account)
            .values(&UserAccount {
                username: "IPFreely".to_string(),
                password: "hunter2".to_string(),
                email: "ipfreely@gmail.com".to_string()
            })
            .get_result::<user_account>(&self.0.get()?)?)
        */
        let user = UserAccount {
            username: "IPFreely".to_string(),
            password: "hunter2".to_string(),
            email: "ipfreely@gmail.com".to_string()
        };

        Ok(user)
    }
}