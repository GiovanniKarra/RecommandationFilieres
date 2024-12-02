use database::{add_course, add_rating, add_student, reset_db};
use rocket::{fs::{relative, NamedFile}, get, launch, routes};
use std::{env, path::{Path, PathBuf}, str::FromStr};
use sqlx::SqlitePool;

mod database;
mod routes;
mod templates;

use routes::*;

#[get("/<path..>")]
async fn serve(path: PathBuf) -> Option<NamedFile> {
	let mut path = Path::new(relative!("static")).join(path);
	if path.is_dir() {
		path.push("index.html");
	}
	path.set_extension("html");

	NamedFile::open(path).await.ok()
}

#[get("/assets/<path..>")]
async fn serve_assets(path: PathBuf) -> Option<NamedFile> {
	let path = Path::new(relative!("assets")).join(path);
	NamedFile::open(path).await.ok()
}


#[launch]
async fn rocket() -> _ {
	let args: Vec<String> = env::args().collect();
	dotenvy::dotenv().ok();
	let database_url = env::var("DATABASE_URL")
		.expect("Expected DATABASE_URL in the environment"); 
	let db_path = PathBuf::from_str("data.db").unwrap();
	
	let schema = include_str!("../database/schema.sql").to_owned();
	let db_pool = match args.contains(&"--reset-db".to_owned()) {
		true => reset_db(db_path, schema)
			.await
			.expect("DB creation error"),
		false => match SqlitePool::connect(&database_url).await {
			Ok(connection) => connection,
			Err(_) => reset_db(db_path, schema)
				.await
				.expect("DB creation error")
		}
	};

	if args.contains(&"--populate-db".to_owned()) {
		for i in 1..5 {
			let _ = add_course(format!("CORS123{i}"), format!("course{i}"), &db_pool).await;
		}
		for i in 1..10 {
			let _ = add_student(format!("stud{}", i), &db_pool).await;
			for j in 1..5 {
				let _ = add_rating(i, j, 2, &db_pool).await;
			}
		}
	}

	let rocket = rocket::build()
		.mount("/", routes![serve, serve_assets])
		.mount("/admin", routes![ratings_matrix_route])
		.manage(db_pool);

	rocket
}
