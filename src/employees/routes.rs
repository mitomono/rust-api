use std::collections::HashMap;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use crate::employees::{Employee, Employees};
use crate::error_handler::CustomError;
use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/employees",
    responses(
        (status = 200, description = "Get all employees", body = inline(response::EmployeesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = web::block(Employees::find_all).await.unwrap();
    Ok(HttpResponse::Ok().json(employees))
}

#[utoipa::path(
    get,
    path = "/get",
    responses(
        (status = 200, description = "Get employees filtered with url params", body = inline(response::EmployeesResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    ),
    params(
        ("id" = Option<i32>, Query, description = "Employee database id"),
        ("ids" = Option<Vec<i32>>, Query, description = "Employees database comma separated ids"),
        ("first_name" = Option<String>, Query,  description = "Employee name"),
        ("last_name" = Option<String>, Query,  description = "Employee last_name"),
        ("department" = Option<String>, Query,  description = "Pet Employee department"),
        ("salary" = Option<f64>, Query, description = "Employee salary"),
        ("age" = Option<i32>, Query, description = "Employee age"),
    )
)]
#[get("/get")]
async fn get(_param: web::Query<HashMap<String, String>>) -> Result<HttpResponse, CustomError> {
    let params = _param.into_inner();

    let employees = if params.is_empty() {
        web::block(Employees::find_all).await.unwrap()
    } else {
        match check::validate_params(&params) {
            Ok(..) => web::block(move || Employees::get(params)).await.unwrap(),
            Err(err) => return Err(err),
        }
    };

    Ok(HttpResponse::Ok().json(employees))
}

#[utoipa::path(
    get,
    path = "/employees/{id}",
    responses(
        (status = 200, description = "Get a employee identifies with id", body = inline(Employees)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/employees/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[utoipa::path(
    post,
    path = "/employees",
    responses(
        (status = 200, description = "Create a new employee", body = inline(response::EmployeeResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/employees")]
async fn create(employee: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[utoipa::path(
    put,
    path = "/employees{id}",
    responses(
    (status = 200, description = "Modify a new employee", body = inline(response::EmployeeResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[put("/employees/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Employee>,
) -> Result<HttpResponse, CustomError> {
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[utoipa::path(
    delete,
    path = "/employees{id}",
    responses(
        (status = 200, description = "Delete a new employee", body = inline(response::DeleteResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(get);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(delete);
}
