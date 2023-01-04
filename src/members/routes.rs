use std::collections::HashMap;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use crate::error_handler::CustomError;
use crate::members::{Member, Members};
use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/members",
    responses(
        (status = 200, description = "Get all members", body = inline(response::MembersResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/members")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let members = web::block(Members::find_all).await.unwrap();
    Ok(HttpResponse::Ok().json(members))
}

#[utoipa::path(
    get,
    path = "/members/filter",
    responses(
        (status = 200, description = "Get members filtered with url params", body = inline(response::MembersResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    ),
    params(
        ("id" = Option<i32>, Query, description = "Member database id"),
        ("ids" = Option<String>, Query, description = "Members database comma separated ids"),
        ("first_name" = Option<String>, Query,  description = "Member name"),
        ("last_name" = Option<String>, Query,  description = "Member last_name"),
        ("email" = Option<String>, Query,  description = "Member email"),
        ("address" = Option<String>, Query, description = "Member address"),
        ("age" = Option<i32>, Query, description = "Member age"),
    )
)]
#[get("/members/filter")]
async fn filter(_param: web::Query<HashMap<String, String>>) -> Result<HttpResponse, CustomError> {
    let params = _param.into_inner();

    let members = if params.is_empty() {
        web::block(Members::find_all).await.unwrap()
    } else {
        match check::validate_members_params(&params) {
            Ok(..) => web::block(move || Members::get(params)).await.unwrap(),
            Err(err) => return Err(err),
        }
    };

    Ok(HttpResponse::Ok().json(members))
}

#[utoipa::path(
    get,
    path = "/members/{id}",
    responses(
        (status = 200, description = "Get a member identifies with id", body = inline(Members)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/members/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let member = Members::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(member))
}

#[utoipa::path(
    post,
    path = "/members",
    responses(
        (status = 200, description = "Create a new member", body = inline(response::MemberResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/members")]
async fn create(member: web::Json<Member>) -> Result<HttpResponse, CustomError> {
    let member = Members::create(member.into_inner())?;
    Ok(HttpResponse::Ok().json(member))
}

#[utoipa::path(
    put,
    path = "/members{id}",
    responses(
    (status = 200, description = "Modify a new member", body = inline(response::MemberResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[put("/members/{id}")]
async fn update(
    id: web::Path<i32>,
    member: web::Json<Member>,
) -> Result<HttpResponse, CustomError> {
    let member = Members::update(id.into_inner(), member.into_inner())?;
    Ok(HttpResponse::Ok().json(member))
}

#[utoipa::path(
    delete,
    path = "/members{id}",
    responses(
        (status = 200, description = "Delete a new member", body = inline(response::DeleteResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[delete("/members/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_member = Members::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_member })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(filter);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(delete);
}
