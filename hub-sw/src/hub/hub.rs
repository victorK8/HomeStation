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
const NUMBER_OF_SENSORS:u8 = 2;

///********************************** DATA STRUCTS ************************************

/// Sensor Data Struct
#[derive(Serialize, Deserialize)]
pub struct Sensor {
	id: String,
 	temperature: f32,
 	humidity: f32,
 	stamp: String,
}

/// Network Of Sensors Struct
#[derive(Serialize)]
pub struct Network {
	things: Vec<Sensor>,
 	number_of_things: u8,
}

/// uri struct for cutting urls
#[derive(Deserialize)]
pub struct Uri {
    id: String,
}

/// Return of a Request Struct
#[derive(Serialize)]
pub struct Response {
    result: bool,
}


///********************************** FUNCTIONS *****************************************


///********************************** HTTP METHODS *************************************

/// Status of a light by id
#[get("/Sensors")]
pub async fn get_sensors(my_uri: web::Query<Uri>) -> impl Responder {

	/// Get sent argument
	let arg = &my_uri.id;

	/// Debug log
	println!("[DEBUG] {}", arg );

	// Logs
    println!("[LOG] Hub module: GET Sensors Status Shadows");

	// Depends of argument
	if arg.len() > 1 {

		/// Check it's "all"
		if arg.ne("all") {

		    // Return as http-response with a json
	   		 HttpResponse::Ok().json(Response{result:false})

		}else{

			/// Array of things
			let mut vector_of_things: Vec<Sensor> = Vec::new();

			/// Loop
			for id in 0..NUMBER_OF_SENSORS{

				// Create dummy thing
				let thing = Sensor{
					id: format!("{}",id),
				 	temperature: 12.22,
				 	humidity: 30.00,
				 	stamp: String::from("dd/mm/yyyy - HH:MM:SS"),
				};

				/// Push
				vector_of_things.push(thing);
			}

			let thing_network = Network{
				things: vector_of_things,
			 	number_of_things: NUMBER_OF_SENSORS,
			};

		    // Return as http-response with a json
		    HttpResponse::Ok().json(thing_network)
		}


	}else if arg.len() == 1 {
        /// It's "id"
        let id:u8 = arg.parse::<u8>().unwrap();

        if id < NUMBER_OF_SENSORS{
			// Status. Some dummy example
			let thing = Sensor{
				id: format!("{}",id),
			 	temperature: 12.22,
			 	humidity: 30.00,
			 	stamp: String::from("dd/mm/yyyy - HH:MM:SS"),
			};

		    // Return as http-response with a json
		    HttpResponse::Ok().json(thing)

		}else{
		    // Return as http-response with a json
	  	 	HttpResponse::Ok().json(Response{result:false})
		}

	}else{
	    // Return as http-response with a json
	    HttpResponse::Ok().json(Response{result:false})
	}





}

