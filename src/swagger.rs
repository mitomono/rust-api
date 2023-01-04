use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::employees;

#[derive(OpenApi)]
#[openapi(
    paths(
        employees::find_all,
        employees::get,
        employees::find,
        employees::create,
        employees::update,
        employees::delete
    ),
    components(schemas(employees::Employees))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    config.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}
