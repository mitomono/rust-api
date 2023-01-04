use std::collections::{HashMap, HashSet};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::members;
use crate::utils::check;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = members)]
pub struct Member {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = members)]
pub struct Members {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
    pub age: i32,
}

impl Members {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let members = members::table.load::<Members>(&mut conn)?;
        Ok(members)
    }

    pub fn get(params: HashMap<String, String>) -> Result<Vec<Self>, CustomError> {
        let mut query = members::table.into_boxed();

        if params.contains_key("id") {
            match check::validate_int(params.get("id").unwrap()) {
                Ok(id) => query = query.filter(members::id.eq(id)),
                Err(error) => return Err(error),
            }
        }
        if params.contains_key("ids") {
            match check::parse_ids(params.get("ids").unwrap()) {
                Ok(ids) => {
                    let ids_clean: HashSet<i32> = ids.into_iter().collect();
                    query = query.filter(members::id.eq_any(ids_clean));
                }
                Err(error) => return Err(error),
            }
        }
        if let Some(first_name) = params.get("first_name") {
            query = query.filter(members::first_name.eq(first_name))
        }
        if let Some(last_name) = params.get("last_name") {
            query = query.filter(members::last_name.eq(last_name))
        }
        if let Some(email) = params.get("email") {
            query = query.filter(members::email.eq(email))
        }
        if let Some(address) = params.get("address") {
            query = query.filter(members::address.eq(address));
        }
        if let Some(age) = params.get("age") {
            match check::validate_int(age) {
                Ok(n) => query = query.filter(members::age.eq(n)),
                Err(err) => return Err(err),
            }
        }

        let mut conn = db::connection()?;
        let members = match query.get_results(&mut conn) {
            Ok(members) => members,
            Err(e) => return Err(CustomError::from(e)),
        };

        Ok(members)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let member = members::table.filter(members::id.eq(id)).first(&mut conn)?;
        Ok(member)
    }

    pub fn create(member: Member) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let member = Member::from(member);
        let member = diesel::insert_into(members::table)
            .values(member)
            .get_result(&mut conn)?;
        Ok(member)
    }

    pub fn update(id: i32, member: Member) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let member = diesel::update(members::table)
            .filter(members::id.eq(id))
            .set(member)
            .get_result(&mut conn)?;
        Ok(member)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(members::table.filter(members::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Member {
    fn from(member: Member) -> Member {
        Member {
            first_name: member.first_name,
            last_name: member.last_name,
            email: member.email,
            address: member.address,
            age: member.age,
        }
    }
}
