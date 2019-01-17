
pub mod permissions;

use jsonwebtoken::{decode, encode, Header, Validation};
use bcrypt::verify;

use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};


use super::dbdata::User;
use super::schema::user_account;
use chrono::{Duration, Local};

pub struct Auth;
trait AuthFunctions {

}

impl AuthFunctions for Auth {

}