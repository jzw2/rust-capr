use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::{fs::write, sync::Mutex};

use crate::trans::transduce_text;

mod trans;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SaveData {
    columns: Vec<Vec<String>>,
    transduceGoal: String,
}

#[derive(Debug)]
struct AppState {
    saves: Mutex<Vec<SaveData>>,
}

#[post("/save")]
async fn save(data: web::Json<SaveData>, state: web::Data<AppState>) -> HttpResponse {
    let mut saves = state.saves.lock().unwrap();
    dbg!(&data);
    println!("old length {}", saves.len());

    let save = data.into_inner();
    saves.push(save.clone());
    dbg!(&saves);
    println!("new length {}", saves.len());

    match write(
        "saves.txt",
        serde_json::to_string(saves.as_slice()).unwrap(),
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return HttpResponse::InternalServerError().body("Failed to save data");
        }
    }

    println!("Received sound laws {:?}", save.clone());

    let text = transduce_text(save.columns, save.transduceGoal);

    println!("Resulted in {}", text);

    HttpResponse::Ok().json(saves.len() - 1)
}

#[get("/load/{id}")]
async fn load(id: web::Path<usize>, state: web::Data<AppState>) -> HttpResponse {
    let saves = state.saves.lock().unwrap();

    dbg!(&saves);
    let get = saves.get(*id);

    if let Some(my_save) = get {
        HttpResponse::Ok().json(my_save)
    } else {
        HttpResponse::NotFound().body(format!("Id {} not found", *id))
    }
}

#[get("/saves")]
async fn get_saves(state: web::Data<AppState>) -> HttpResponse {
    let saves = state.saves.lock().unwrap();
    HttpResponse::Ok().json(saves.as_slice())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        saves: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(save)
            .service(load)
            .service(get_saves)
            .service(Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
