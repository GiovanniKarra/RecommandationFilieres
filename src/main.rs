use database::reset_db;
use rocket::{fs::{relative, NamedFile}, launch};
use std::{env, path::{Path, PathBuf}, str::FromStr};
use sqlx::SqlitePool;

mod database;


#[rocket::get("/<path..>")]
pub async fn serve(mut path: PathBuf) -> Option<NamedFile> {
	path.set_extension("html");
	let mut path = Path::new(relative!("static")).join(path);
	if path.is_dir() {
		path.push("index.html");
	}

	NamedFile::open(path).await.ok()
}

#[rocket::get("/assets/<path..>")]
pub async fn serve_assets(path: PathBuf) -> Option<NamedFile> {
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

	let rocket = rocket::build()
		.mount("/", rocket::routes![serve, serve_assets])
		.manage(db_pool);

	rocket
}
