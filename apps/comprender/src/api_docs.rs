use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Composition Render API",
        description = "todo",
        contact(name = "dyn.art", url = "https://dyn.art/?source=apidocs"),
        version = "1.0.0"
    ),
    paths(crate::routes::v1::render::controller::render_composition)
)]
pub struct ApiDocs;

impl ApiDocs {
    pub fn generate() -> String {
        ApiDocs::openapi().to_json().unwrap()
    }
}
