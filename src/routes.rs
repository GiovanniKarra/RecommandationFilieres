use rocket::{form::Form, get, post, FromForm, State};
use sqlx::SqlitePool;

use crate::{database::{add_course, get_courses, get_ratings_matrix, get_students}, templates::{CoursesList, RatingsMatrix}};


#[get("/matrix")]
pub async fn ratings_matrix_route(pool: &State<SqlitePool>) -> Result<RatingsMatrix, String> {
	let matrix = get_ratings_matrix(pool).await?;

	let courses = get_courses(pool).await?;
	let students = get_students(pool).await?;

	Ok(RatingsMatrix{ courses, students, matrix: matrix.to_vec_vec() })
}

#[get("/courses")]
pub async fn course_list_route(pool: &State<SqlitePool>) -> Result<CoursesList, String> {
	let courses = get_courses(pool).await?;
	Ok(CoursesList{ courses })
}


#[derive(FromForm)]
pub struct NewCourse {
	pub code: String,
	pub name: String
}

#[post("/new-course", data = "<course_data>")]
pub async fn add_course_route(course_data: Form<NewCourse>, pool: &State<SqlitePool>) -> Result<(), String> {
	add_course(course_data.code.clone(), course_data.name.clone(), pool).await?;
	Ok(())
}