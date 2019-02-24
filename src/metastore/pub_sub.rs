
use diesel::prelude::*;
use diesel;


use data::channels::Channels;
use data::channels::Subscription;
use metastore::schema;
use metastore::dbdata;
use connection::executor::Conn;
use diesel::result::Error as DbError;
use diesel::result::DatabaseErrorKind as DbErrKind;
use data::auth::User;

use state::error::BroadcastError;
use state::PubSubOps;
use state::PublishCallback;
use data::Message;
use diesel::types;

impl<'a> PubSubOps for PublishCallback<'a> {

    fn publish(&self, channel: Channels, action_name: String, action_result: &serde_json::Value) -> Result<(), BroadcastError> {

        let raw_channel = get_or_create_channel(self.conn, &channel)?;

        let raw_message = dbdata::NewRawMessage {
            channel_id: raw_channel.channel_id,
            data: action_result.to_owned(),
        };

        diesel::insert_into(schema::message::table)
            .values(&raw_message)
            .execute(self.conn)
            .map_err(|err| {
                println!("Could not get or create err: {:?}", &err);

                BroadcastError::InternalError(err.to_string())
            })?;

        Ok(())
    }

    fn subscribe(&self, user_id: String, channel: Channels) -> Result<Subscription, BroadcastError> {
        info!("subscribing to channels: {:?}", &channel);

        let raw_user = get_user(self.conn, &user_id)?;
        let raw_channel = get_or_create_channel(self.conn, &channel)?;
        let raw_user_channel = create_user_channel(self.conn, raw_user.user_id, raw_channel.channel_id)?;

        let user = User {
            username: raw_user.username,
            email: raw_user.email,
            display_name: raw_user.display_name,
        };

        Ok(Subscription { user, channel })
    }

    fn unsubscribe(&self, user_id: String, channel: Channels) -> Result<Subscription, BroadcastError> {
        info!("unsubscribing from channels: {:?}", &channel);

        let raw_user = get_user(self.conn, &user_id)?;
        let raw_channel = get_channel(self.conn, &channel)?;
        let raw_user_channel = remove_user_channel(self.conn, raw_user.user_id, raw_channel.channel_id)?;

        let user = User {
            username: raw_user.username,
            email: raw_user.email,
            display_name: raw_user.display_name,
        };

        Ok(Subscription { user, channel })
    }

    fn get_subscribers(&self, channel: Channels) -> Result<Vec<User>, BroadcastError> {
        info!("getting all subscribers from channels: {:?}", &channel);

        let query = r#"
        SELECT
            DISTINCT ON("user"."user_id")
            "user".* FROM "user"
        INNER JOIN "user_channel"
            ON "user"."user_id" = "user_channel"."user_id"
        INNER JOIN "channel"
            ON "user_channel"."channel_id" = "channel"."channel_id"
        WHERE "channel"."data" = $1;
        "#;

        let channel_json = serde_json::to_value(&channel)
            .map_err(|err| {
                error!("Could not serialize value {:?} error: {:?}", &channel, &err);
                BroadcastError::Unknown
            })?;
        let raw_users: Vec<dbdata::RawUser> = diesel::sql_query(query)
            .bind::<types::Json, _>(&channel_json)
            .load(self.conn)
            .map_err(|err| BroadcastError::InternalError(err.to_string()))?;

        let users: Vec<User> = raw_users
            .into_iter()
            .map(|raw_user| User {
                username: raw_user.username,
                email: raw_user.email,
                display_name: raw_user.display_name,
            })
            .collect();

        Ok(users)
    }

    fn get_messages(
        &self,
        channel: Channels,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Result<Vec<Message>, BroadcastError> {
        let query = r#"
        SELECT
            "message".* FROM "message"
        INNER JOIN "channel"
            ON "message"."channel_id" = "channel"."channel_id"
        WHERE "channel"."data" = $1 AND "message"."sent_at" >= $2 AND "message"."sent_at" < $3
        ORDER BY "message"."sent_at" DESC;
        "#;

        let channel_json = serde_json::to_value(&channel)
            .map_err(|err| {
                error!("Could not serialize value {:?} error: {:?}", &channel, &err);
                BroadcastError::Unknown
            })?;

        let raw_messages: Vec<dbdata::RawMessage> = diesel::sql_query(query)
            .bind::<types::Json, _>(&channel_json)
            .bind::<types::Timestamp, _>(&start_time)
            .bind::<types::Timestamp, _>(&end_time)
            .load(self.conn)
            .map_err(|err| BroadcastError::InternalError(err.to_string()))?;


        let messages: Vec<Message> = raw_messages
            .into_iter()
            .map(|raw_message| Message {
                data: raw_message.data,
                timestamp: raw_message.sent_at,
            })
            .collect();

        Ok(messages)

    }

    fn permissions_removed(&self) -> Result<(), BroadcastError> {
        unimplemented!()
    }
}

fn get_user(conn: &Conn, user_identifier: &String) -> Result<dbdata::RawUser, BroadcastError> {
    debug!("Authenticating user: {:?}", user_identifier);
    schema::user::table
        .filter(schema::user::columns::username.eq(&user_identifier))
        .or_filter(schema::user::columns::email.eq(&user_identifier))
        .get_result::<dbdata::RawUser>(conn)
        .map_err(|err| {
            info!("Could not find user: {:?}", &user_identifier);
            match err {
                DbError::NotFound => BroadcastError::UserNotFound,
                _ => BroadcastError::InternalError(err.to_string()),
            }
        })

}

fn get_or_create_channel(conn: &Conn, channel: &Channels) -> Result<dbdata::RawChannel, BroadcastError> {
    let channel_json = serde_json::to_value(channel)
        .map_err(|err| {
            error!("Could not serialize value {:?} error: {:?}", &channel, &err);
            BroadcastError::Unknown
        })?;
    let channel_value = dbdata::NewRawChannel {
        data: channel_json,
    };

    schema::channel::table
        .filter(schema::channel::columns::data.eq(&channel_value.data))
        .get_result::<dbdata::RawChannel>(conn)
        .or_else(|err| match err {
            DbError::NotFound => {
                diesel::insert_into(schema::channel::table)
                    .values(&channel_value)
                    .get_result::<dbdata::RawChannel>(conn)
            },
            _ => Err(err),
        })
        .map_err(|err| {
            println!("Could not get or create err: {:?}", &err);

            BroadcastError::InternalError(err.to_string())
        })
}

fn get_channel(conn: &Conn, channel: &Channels) -> Result<dbdata::RawChannel, BroadcastError> {
    let channel_json = serde_json::to_value(channel)
        .map_err(|err| {
            error!("Could not serialize value {:?} error: {:?}", &channel, &err);
            BroadcastError::Unknown
        })?;
    let channel_value = dbdata::NewRawChannel {
        data: channel_json,
    };

    schema::channel::table
        .filter(schema::channel::columns::data.eq(&channel_value.data))
        .get_result::<dbdata::RawChannel>(conn)
        .map_err(|err| match err {
            DbError::NotFound => BroadcastError::NotSubscribed,
            _ => BroadcastError::InternalError(err.to_string()),
        })
}

fn create_user_channel(conn: &Conn, user_id: i64, channel_id: i64) -> Result<dbdata::RawUserChannel, BroadcastError> {
    let user_channel_value = dbdata::NewRawUserChannel { user_id, channel_id, };

    diesel::insert_into(schema::user_channel::table)
        .values(&user_channel_value)
        .get_result::<dbdata::RawUserChannel>(conn)
        .map_err(|err| match err {
            DbError::DatabaseError(DbErrKind::UniqueViolation, _) => {
                BroadcastError::AlreadySubscribed
            },
            _ => {
                BroadcastError::InternalError(err.to_string())
            }
        })
}

fn remove_user_channel(conn: &Conn, user_id: i64, channel_id: i64) -> Result<dbdata::RawUserChannel, BroadcastError> {
    let user_channel_value = dbdata::NewRawUserChannel { user_id, channel_id, };

    diesel::delete(schema::user_channel::table)
        .filter(schema::user_channel::columns::user_id.eq(&user_id))
        .filter(schema::user_channel::columns::channel_id.eq(&channel_id))
        .get_result::<dbdata::RawUserChannel>(conn)
        .map_err(|err| match err {
            DbError::NotFound => {
                BroadcastError::NotSubscribed
            },
            _ => {
                BroadcastError::InternalError(err.to_string())
            }
        })
}