use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use std::{fs::write, sync::Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SaveData {
    columns: Vec<Vec<String>>,
}

#[derive(Debug)]
struct AppState {
    saves: Mutex<Vec<SaveData>>,
}

#[post("/save")]
async fn save(data: web::Json<SaveData>, state: web::Data<AppState>) -> HttpResponse {
    let mut saves = state.saves.lock().unwrap();
    saves.push(data.into_inner());


    match write("saves.txt", serde_json::to_string(saves.as_slice()).unwrap()) {
        Ok(_) => {
        }
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return HttpResponse::InternalServerError().body("Failed to save data");
        }
    }

    HttpResponse::Ok().json(saves.len() - 1)
}

#[get("/load/{id}")]
async fn load(id: web::Path<usize>, state: web::Data<AppState>) -> HttpResponse {

    let saves = state.saves.lock().unwrap();

    dbg!("{}", &saves);
    let get = saves.get(*id);

    if let Some(my_save) = get {
        HttpResponse::Ok().json(my_save)
    } else {
        HttpResponse::NotFound().body("Save not found")
    }
}

#[get("/saves")]
async fn get_saves(state: web::Data<AppState>) -> HttpResponse {
    let saves = state.saves.lock().unwrap();
    HttpResponse::Ok().json(saves.as_slice())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                saves: Mutex::new(Vec::new()),
            }))
            .service(save)
            .service(load)
            .service(get_saves)
            .service(Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
