use rocket::{form::Form, get, post, FromForm, State};
use sqlx::SqlitePool;

use crate::{database::{add_course, add_student, get_courses, get_ratings_matrix, get_students, get_types}, templates::{CoursesList, RatingsForm, RatingsMatrix, TypeSelection}};


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

#[get("/course-type-select")]
pub async fn course_types_select_route(pool: &State<SqlitePool>) -> Result<TypeSelection, String> {
	let types = get_types(pool).await?;
	Ok(TypeSelection { types })
}


#[derive(FromForm)]
pub struct NewCourse {
	pub code: String,
	pub name: String,
	pub r#type: String
}
#[post("/new-course", data = "<course_data>")]
pub async fn add_course_route(course_data: Form<NewCourse>, pool: &State<SqlitePool>) -> Result<(), String> {
	add_course(course_data.code.clone(), course_data.name.clone(), course_data.r#type.clone(), pool).await?;
	Ok(())
}

#[derive(FromForm)]
pub struct StudentData {
	pub name: String,
	pub type1: String,
	pub type2: String
}
#[post("/form", data = "<student_data>")]
pub async fn add_student_route(student_data: Form<StudentData>, pool: &State<SqlitePool>) -> Result<RatingsForm, String> {
	add_student(student_data.name.clone(), pool).await?;
	let courses = get_courses(pool).await?;
	Ok(RatingsForm { courses })
}

