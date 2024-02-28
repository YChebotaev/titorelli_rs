use actix_web::{self, body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ClassificationResult {
    pub r#type: String,
    pub score: f64,
}

impl Responder for ClassificationResult {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_error) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{\"error\": \"Failed to serialize\"}"),
        }
    }
}
