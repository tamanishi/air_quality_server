use actix_web::{web, App, HttpResponse, Error, HttpServer};
use std::sync::Mutex;

mod ccs811;
mod data;

async fn measure(ccs811: web::Data<Mutex<ccs811::Ccs811>>, env: web::Json<data::Environment>) -> Result<HttpResponse, Error> {
    let ccs811 = ccs811.lock().unwrap();
    match ccs811.read(&env) {
        Ok(data) => {
            Ok(HttpResponse::Ok().json(data))
        },
        Err(e) => {
            ccs811.activate();
            let res = HttpResponse::InternalServerError().json(e);
            Ok(res)
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ccs811 = ccs811::Ccs811::new();
    ccs811.activate();
    let ccs811_wrapper = web::Data::new(Mutex::new(ccs811));
    HttpServer::new(move || {
        App::new()
            .app_data(ccs811_wrapper.clone())
            .service(web::resource("/air_quality").to(measure))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
