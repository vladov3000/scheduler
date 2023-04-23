use actix_files::Files;
use actix_web::{middleware::Logger, post, web::{Redirect, Data}, App, HttpServer, Responder};
use rusqlite::{Connection, Statement};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Serialize, Deserialize)]
enum Day {
    Monday,
    Tuesday,
    Wednsday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize, Deserialize)]
struct Time {
    day: Day,
    half_hours: i32,
}

#[derive(Serialize, Deserialize)]
struct Span {
    start: Time,
    end: Time,
}

#[derive(Serialize, Deserialize)]
struct Schedule {
    span: Span,
    available: HashMap<String, Vec<Span>>,
}

#[post("/api/schedule")]
async fn schedule(database: Data<Arc<Mutex<Database>>>) -> impl Responder {
    let database = database.lock();

    
    
    Redirect::to("/").see_other()
}

#[derive(Send)]
struct Database {
    connection: Connection,
    insert_schedule_statement: Statement,
}

impl Database {
    fn new() -> Self {
	let connection = Connection::open("database/scheduler.db").unwrap();
	let insert_schedule_statement = connection.prepare("SELECT id FROM schedule").unwrap();

	Database {
	    connection
	}
    }
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    let database = Arc::new(Mutex::new(Database::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service((
                schedule,
                Files::new("/", "./static").index_file("index.html"),
            ))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
    .unwrap()
}
