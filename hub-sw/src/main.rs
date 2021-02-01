/// Main of Hub-Server App
/// 
/// Server App
///
/// By Victor M. - CIRCE - Based on the next page
///

use actix_web::{web, App, HttpServer}; /// Actix Framework Pkgs.


mod hub {
    pub mod hub;
}

mod user {
	pub mod user;
}

mod webpage {
    pub mod backend;
}

///********************************** SERVER MAIN *****************************************

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	println!("[LOG] Main: Running Server");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/User/")
                    .service(user::user::execute_command)
                    .service(user::user::check_user)
            )
            .service(
                web::scope("/Hub/")
                    .service(hub::hub::sensors_by_id)
                    .service(hub::hub::all_sensors)
            )
            .service(webpage::backend::index)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

