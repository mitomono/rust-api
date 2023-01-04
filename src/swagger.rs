use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::books;
use crate::members;

#[derive(OpenApi)]
#[openapi(
    paths(
        members::find_all,
        members::filter,
        members::find,
        members::create,
        members::update,
        members::delete,
        books::find_all,
        books::filter,
        books::find,
        books::create,
        books::update,
        books::delete
    ),
    components(schemas(members::Members), schemas(books::Books))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    config.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}
