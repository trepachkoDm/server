use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Room {
    name: String,
    devices: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct House {
    name: String,
    rooms: Vec<Room>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceState {
    room_name: String,
    device_name: String,
    state: String,
}

// #[warn(dead_code)]
// fn get_house() -> House {
//     // this function should retrieve the house structure from the database
//     // for the purpose of this example, we'll just return a mock object
//     House {
//         name: "My House".to_string(),
//         rooms: vec![
//             Room {
//                 name: "Living Room".to_string(),
//                 devices: vec!["TV".to_string(), "Sound System".to_string()],
//             },
//             Room {
//                 name: "Bedroom".to_string(),
//                 devices: vec!["Lamp".to_string(), "Alarm Clock".to_string()],
//             },
//         ],
//     }
// }

async fn get_device_state(_: web::Path<(String, String)>) -> impl Responder {
    let state = "on".to_string();
    HttpResponse::Ok().json(DeviceState {
        room_name: "Living Room".to_string(),
        device_name: "TV".to_string(),
        state,
    })
}

async fn add_room(room: web::Json<Room>) -> impl Responder {
    format!("Added room {} to the house", room.name)
}

async fn remove_room(room_name: web::Path<String>) -> impl Responder {
    format!("Removed room {} from the house", room_name)
}

async fn add_device(
    room_name: web::Path<String>,
    device_name: web::Json<String>,
) -> impl Responder {
    format!("Added device {} to room {}", device_name, room_name)
}

async fn remove_device(
    room_name: web::Path<String>,
    device_name: web::Path<String>,
) -> impl Responder {
    format!("Removed device {} from room {}", device_name, room_name)
}

async fn get_rooms() -> impl Responder {
    let rooms = vec!["Living Room".to_string(), "Bedroom".to_string()];
    HttpResponse::Ok().json(rooms)
}

async fn get_devices(room_name: web::Path<String>) -> impl Responder {
    let devices = match room_name.as_str() {
        "Living Room" => vec!["TV".to_string(), "Sound System".to_string()],
        "Bedroom" => vec!["Lamp".to_string(), "Alarm Clock".to_string()],
        _ => vec![],
    };
    HttpResponse::Ok().json(devices)
}

async fn get_report<T: ToString>() -> impl Responder {
    let report = vec![
        DeviceState {
            room_name: "Living Room".to_string(),
            device_name: "TV".to_string(),
            state: "on".to_string(),
        },
        DeviceState {
            room_name: "Living Room".to_string(),
            device_name: "Sound System".to_string(),
            state: "off".to_string(),
        },
        DeviceState {
            room_name: "Bedroom".to_string(),
            device_name: "Lamp".to_string(),
            state: "on".to_string(),
        },
        DeviceState {
            room_name: "Bedroom".to_string(),
            device_name: "Alarm Clock".to_string(),
            state: "off".to_string(),
        },
    ];
    HttpResponse::Ok().json(report)
}

async fn index() -> impl Responder {
    "Welcome to the Smart Home API!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .unwrap();
    let database = client.database("smart_home");
    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/", web::get().to(index))
            .route("/rooms", web::get().to(get_rooms))
            .route("/rooms", web::post().to(add_room))
            .route("/rooms/{room_name}", web::delete().to(remove_room))
            .route("/rooms/{room_name}/devices", web::get().to(get_devices))
            .route("/rooms/{room_name}/devices", web::post().to(add_device))
            .route(
                "/rooms/{room_name}/devices/{device_name}",
                web::delete().to(remove_device),
            )
            .route("/report/{device_type}", web::get().to(get_report::<String>))
            .route(
                "/report/{device_type}/{format}",
                web::get().to(get_report::<String>),
            )
            .route(
                "/state/{room_name}/{device_name}",
                web::get().to(get_device_state),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
