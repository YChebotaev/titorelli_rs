mod classification_example;
mod classification_result;
mod model;
mod train_example;
use actix_web::{self, post, web, App, HttpResponse, HttpServer, Responder};
use classification_example::ClassificationExample;
use classification_result::ClassificationResult;
use model::{ClassifiedType, ExampleType, Model};
use std::sync::{Arc, Mutex};
use train_example::TrainExample;

struct AppState {
    model: Arc<Mutex<Model>>,
}

#[post("/classify")]
async fn classify(
    data: web::Data<AppState>,
    body: web::Json<ClassificationExample>,
) -> impl Responder {
    let model = data.model.lock().unwrap();

    match model.classify(&body.text) {
        ClassifiedType::Spam(s) => ClassificationResult {
            r#type: "spam".into(),
            score: s,
        },
        ClassifiedType::Ham(s) => ClassificationResult {
            r#type: "ham".into(),
            score: s,
        },
    }
}

#[post("/train_bulk")]
async fn train_bulk(
    data: web::Data<AppState>,
    body: web::Json<Vec<TrainExample>>,
) -> impl Responder {
    let mut model = data.model.lock().unwrap();
    let train_examples = body.to_vec();

    for example in train_examples {
        if example.r#type == "spam" {
            model.train(ExampleType::Spam, &example.text);
        } else if example.r#type == "ham" {
            model.train(ExampleType::Ham, &example.text);
        }
    }

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let model = Arc::new(Mutex::new(Model::new()));

    HttpServer::new(move || {
        let app_state = AppState {
            model: model.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(classify)
            .service(train_bulk)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
