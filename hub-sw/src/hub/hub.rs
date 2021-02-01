/// EModule for sensor's hub purposes
/// 
/// Hub Module 
///
/// By Victor M. - CIRCE 

use actix_web::{get, post, web, HttpResponse, Responder}; /// Actix Framework Pkgs.
use serde::{Serialize, Deserialize}; /// Serde Pkg.
use std::fs::File; /// File handling pkg
use std::io::prelude::*;
use std::string::String; // String pkg.

///********************************** CONST ************************************
const NUMBER_OF_SENSORS:usize = 2;

///********************************** DATA STRUCTS ************************************

/// Sensor Struct
#[derive(Serialize)]
pub struct Sensor {
	id: u8,
 	temperature: f32,
 	humidity: f32,
 	stamp: String,
}

/// Network Struct
#[derive(Serialize)]
pub struct Network {
	things: [Sensor;NUMBER_OF_SENSORS],
 	number_of_things: u8,
}
/// Result Struct
#[derive(Serialize)]
pub struct Response {
    result: bool,
}


///********************************** FUNCTIONS *****************************************


///********************************** HTTP METHODS *************************************

/// Status of a light by id
#[get("/Sensors")]
pub async fn all_sensors() -> impl Responder {

	// Status. Some dummy example
	let status = Sensor{
		id: 0,
	 	temperature: 12.22,
	 	humidity: 22.22,
	 	stamp: String::from("dd/mm/yyyy - HH:MM:SS"),
	};

	// Logs
    println!("[LOG] Hub module: All Sensors Status Shadows ");

    // Return as http-response with a json
    HttpResponse::Ok().json(status)

}

/// Status of a light by id
#[get("/Sensors/{id}")]
pub async fn sensors_by_id(path: web::Path<(u8,)>) -> impl Responder {

	// Get id from url
	let id_of_sensor:u8 = path.0;

	// Status. Some dummy example
	let status = Sensor{
		id: id_of_sensor,
	 	temperature: 12.22,
	 	humidity: 22.22,
	 	stamp: String::from("dd/mm/yyyy - HH:MM:SS"),

	};

	// Logs
    println!("[LOG] Hub module: {} Sensor Status Shadow ", id_of_sensor);

    // Return as http-response with a json
    HttpResponse::Ok().json(status)

}