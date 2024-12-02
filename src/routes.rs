use rocket::{get, State};
use sqlx::SqlitePool;

use crate::{database::{get_courses, get_ratings_matrix, get_students}, templates::RatingsMatrix};


#[get("/matrix")]
pub async fn ratings_matrix_route(pool: &State<SqlitePool>) -> Result<RatingsMatrix, String> {
	let matrix = get_ratings_matrix(pool).await?;

	let courses = get_courses(pool).await?;
	let students = get_students(pool).await?;

	Ok(RatingsMatrix{ courses, students, matrix: matrix.to_vec_vec() })
}