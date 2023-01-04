use std::collections::{HashMap, HashSet};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::employees;
use crate::utils::check;
use crate::utils::check::{validate_float, validate_int};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = employees)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: f64,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = employees)]
pub struct Employees {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: f64,
    pub age: i32,
}

impl Employees {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let employees = employees::table.load::<Employees>(&mut conn)?;
        Ok(employees)
    }

    pub fn get(params: HashMap<String, String>) -> Result<Vec<Self>, CustomError> {
        let mut query = employees::table.into_boxed();

        if params.contains_key("id") {
            match check::validate_int(params.get("id").unwrap()) {
                Ok(id) => query = query.filter(employees::id.eq(id)),
                Err(error) => return Err(error),
            }
        }
        if params.contains_key("ids") {
            match check::parse_ids(params.get("ids").unwrap()) {
                Ok(ids) => {
                    let ids_clean: HashSet<i32> = ids.into_iter().collect();
                    query = query.filter(employees::id.eq_any(ids_clean));
                }
                Err(error) => return Err(error),
            }
        }
        if let Some(first_name) = params.get("first_name") {
            query = query.filter(employees::first_name.eq(first_name))
        }
        if let Some(last_name) = params.get("last_name") {
            query = query.filter(employees::last_name.eq(last_name))
        }
        if let Some(department) = params.get("department") {
            query = query.filter(employees::department.eq(department))
        }
        if let Some(salary) = params.get("salary") {
            match validate_float(salary) {
                Ok(n) => query = query.filter(employees::salary.eq(n)),
                Err(err) => return Err(err),
            }
        }
        if let Some(age) = params.get("age") {
            match validate_int(age) {
                Ok(n) => query = query.filter(employees::age.eq(n)),
                Err(err) => return Err(err),
            }
        }

        let mut conn = db::connection()?;
        let employees = match query.get_results(&mut conn) {
            Ok(employees) => employees,
            Err(e) => return Err(CustomError::from(e)),
        };

        Ok(employees)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let employee = employees::table
            .filter(employees::id.eq(id))
            .first(&mut conn)?;
        Ok(employee)
    }

    pub fn create(employee: Employee) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let employee = Employee::from(employee);
        let employee = diesel::insert_into(employees::table)
            .values(employee)
            .get_result(&mut conn)?;
        Ok(employee)
    }

    pub fn update(id: i32, employee: Employee) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let employee = diesel::update(employees::table)
            .filter(employees::id.eq(id))
            .set(employee)
            .get_result(&mut conn)?;
        Ok(employee)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res =
            diesel::delete(employees::table.filter(employees::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Employee {
    fn from(employee: Employee) -> Employee {
        Employee {
            first_name: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary: employee.salary,
            age: employee.age,
        }
    }
}
