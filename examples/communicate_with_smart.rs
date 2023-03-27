use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/rooms")
        .send()
        .await?
        .text()
        .await?;
    println!("Rooms: {}", response);

    let room = json!({
        "name": "Kitchen",
        "devices": ["Refrigerator", "Oven"]
    });
    let response = client
        .post("http://localhost:8080/rooms")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&room)
        .send()
        .await?
        .text()
        .await?;
    println!("Add room response: {}", response);

    let response = client
        .get("http://localhost:8080/rooms/Living Room/devices")
        .send()
        .await?
        .text()
        .await?;
    println!("Living Room devices: {}", response);

    let device = json!("Smart Speaker");
    let response = client
        .post("http://localhost:8080/rooms/Living Room/devices")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&device)
        .send()
        .await?
        .text()
        .await?;
    println!("Add device response: {}", response);

    let response = client
        .get("http://localhost:8080/rooms/Living Room/devices/TV")
        .send()
        .await?
        .text()
        .await?;
    println!("TV state: {}", response);

    let state = json!({"state": "off"});
    let response = client
        .put("http://localhost:8080/rooms/Living Room/devices/TV")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .json(&state)
        .send()
        .await?
        .text()
        .await?;
    println!("Set TV state response: {}", response);

    let response = client
        .get("http://localhost:8080/report")
        .send()
        .await?
        .text()
        .await?;
    println!("Smart home report: {}", response);

    let response = client
        .delete("http://localhost:8080/rooms/Kitchen")
        .send()
        .await?
        .text()
        .await?;
    println!("Remove room response: {}", response);

    Ok(())
}

// GET /: Returns a welcome message.
// GET /rooms: Returns a list of all rooms in the house.
// POST /rooms: Adds a new room to the house.
// DELETE /rooms/{room_name}: Removes the specified room from the house.
// GET /rooms/{room_name}/devices: Returns a list of all devices in the specified room.
// POST /rooms/{room_name}/devices: Adds a new device to the specified room.
// DELETE /rooms/{room_name}/devices/{device_name}: Removes the specified device from the specified room.
// GET /devices/{room_name}/{device_name}: Returns the state of the specified device.
// GET /report: Generates a report on the state of the entire house.
// The API is backed by a MongoDB database, and the code defines a Client object that connects to the database. The API implementation stubs provide basic functionality for retrieving and modifying data in the database, but they currently return mock data instead of actually interacting with the database.
