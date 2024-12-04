use rocket::{data::ToByteUnit, form::Form, get, http::{Cookie, CookieJar}, post, response::Redirect, time::{Date, OffsetDateTime, Time}, Data, FromForm, State};
use sqlx::SqlitePool;

use crate::{database::{add_course, add_rating, add_student, get_course_id, get_courses, get_ratings_matrix, get_student_id, get_students, get_types, reset_student_ratings}, templates::{CoursesList, RatingsForm, RatingsMatrix, TypeSelection}};


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
pub async fn add_student_route(student_data: Form<StudentData>, pool: &State<SqlitePool>, jar: &CookieJar<'_>) -> Result<RatingsForm, String> {
	jar.add(
		Cookie::build(("name", student_data.name.clone()))
		.expires(OffsetDateTime::new_utc(
			Date::from_calendar_date(2027, rocket::time::Month::December, 30).unwrap(),
			Time::from_hms(12, 0, 0).unwrap())
		)
	);
	add_student(student_data.name.clone(), pool).await?;
	let courses = get_courses(pool)
		.await?
		.into_iter()
		.filter(|course| course.r#type == student_data.type1 || course.r#type == student_data.type2)
		.collect();
	Ok(RatingsForm { courses })
}

#[post("/rate", data = "<rating_data>")]
pub async fn post_rating_route(rating_data: Data<'_>, pool: &State<SqlitePool>, jar: &CookieJar<'_>) -> Result<Redirect, String> {
	let student_name = jar.get("name")
		.ok_or("Name not found in cookies")?
		.value()
		.to_owned();
	let student_id = get_student_id(student_name, pool).await?;
	reset_student_ratings(student_id, pool).await?;

	let pairs: Vec<(String, u8)> = rating_data.open(1.megabytes())
		.into_string()
		.await
		.map_err(|e| e.to_string())?
		.split("&")
		.map(|elem| -> Result<(String, u8), String> {
			elem.split_once("=")
				.map(|elem| elem.1.parse::<u8>()
						.map(|n| (elem.0.to_owned(), n))
						.map_err(|err| err.to_string())
				)
				.ok_or("Couldn't parse request".to_owned())?
		})
		.collect::<Result<Vec<(String, u8)>, String>>()?;

	for (code, rating) in pairs {
		let course_id = get_course_id(code, pool).await?;
		add_rating(student_id.clone(), course_id, rating as i64, pool).await?;
	}

	Ok(Redirect::to("/"))
}