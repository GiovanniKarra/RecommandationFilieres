use std::path::PathBuf;

use std::fs::File;
use sqlx::SqlitePool;

pub async fn reset_db(path: PathBuf, schema: String) -> Result<SqlitePool, String> {
	let _file = File::create(&path).map_err(|e| e.to_string())?;
	let db_url = "sqlite://".to_owned() + path
		.to_str()
		.ok_or("Can't convert path to string")?;

	let pool = SqlitePool::connect(&db_url)
		.await
		.map_err(|e| e.to_string())?;

	sqlx::query(&schema)
		.execute(&pool)
		.await
		.map_err(|e| e.to_string())?;

	Ok(pool)
}