use std::path::PathBuf;

use std::fs::File;
use sqlx::SqlitePool;

use libmatcompl::FMatrix;


#[derive(Debug)]
pub struct Student {
	pub id: i64,
	pub name: String
}

#[derive(Debug)]
pub struct Course {
	pub id: i64,
	pub code: String,
	pub name: String,
	pub r#type: String
}

#[derive(Debug)]
pub struct Rating {
	pub id: i64,
	pub student: i64,
	pub course: i64,
	pub rating: i64
}

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


pub async fn add_student(name: String, pool: &SqlitePool) -> Result<(), String> {
	let student = sqlx::query_as!(
		Student,
		"
		SELECT id, name FROM students
		WHERE name = ?;
		",
		name
	)
	.fetch_optional(pool)
	.await
	.map_err(|e| e.to_string())?;

	if student.is_some() { return Ok(()) }
	
	sqlx::query!(
		"
		INSERT INTO students (name)
		VALUES (?);
		",
		name
	)
	.execute(pool)
	.await
	.map_err(|e| e.to_string())?;
	
	Ok(())
}


pub async fn add_course(code: String, name: String, r#type: String, pool: &SqlitePool) -> Result<(), String> {
	let course = sqlx::query_as!(
		Course,
		"
		SELECT id, code, name, type
		FROM courses
		WHERE name = ? AND code = ?;
		",
		name, code
	)
	.fetch_optional(pool)
	.await
	.map_err(|e| e.to_string())?;

	if course.is_some() { return Err("Course already exists".to_owned()) }
	
	sqlx::query!(
		"
		INSERT INTO courses (code, name, type)
		VALUES (?, ?, ?);
		",
		code, name, r#type
	)
	.execute(pool)
	.await
	.map_err(|e| e.to_string())?;
	
	Ok(())
}


pub async fn add_rating(student_id: i64, course_id: i64, rating: i64, pool: &SqlitePool) -> Result<(), String> {
	sqlx::query!(
		"
		INSERT INTO ratings (student, course, rating)
		VALUES (?, ?, ?);
		",
		student_id, course_id, rating
	)
	.execute(pool)
	.await
	.map_err(|e| e.to_string())?;
	
	Ok(())
}

pub async fn get_student_id(name: String, pool: &SqlitePool) -> Result<i64, String> {
	sqlx::query_as!(
		Student,
		"
		SELECT id, name FROM students
		WHERE name = ?;
		",
		name
	)
	.fetch_optional(pool)
	.await
	.map_err(|e| e.to_string())?
	.map(|s| s.id)
	.ok_or("Couldn't fetch student id, probably incorrect name".to_owned())
}

pub async fn get_course_id(code: String, pool: &SqlitePool) -> Result<i64, String> {
	sqlx::query_as!(
		Course,
		"
		SELECT id, code, name, type FROM courses
		WHERE code = ?;
		",
		code
	)
	.fetch_optional(pool)
	.await
	.map_err(|e| e.to_string())?
	.map(|s| s.id)
	.ok_or("Couldn't fetch course id, probably incorrect code".to_owned())
}

pub async fn get_student_ratings(student_id: i64, pool: &SqlitePool) -> Result<Vec<Rating>, String> {
	sqlx::query_as!(
		Rating,
		"
		SELECT id, student, course, rating
		FROM ratings
		WHERE student = ?;
		",
		student_id
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())
}

pub async fn reset_student_ratings(student_id: i64, pool: &SqlitePool) -> Result<(), String> {
	sqlx::query_as!(
		Rating,
		"
		DELETE FROM ratings
		WHERE student = ?;
		",
		student_id
	)
	.execute(pool)
	.await
	.map_err(|e| e.to_string())?;

	Ok(())
}

pub async fn get_ratings_matrix(pool: &SqlitePool) -> Result<FMatrix, String> {
	let m = get_student_count(pool).await?;
	let n = get_course_count(pool).await?;

	let mut matrix = FMatrix::empty(m, n);

	let ratings = sqlx::query_as!(
		Rating,
		"
		SELECT id, student, course, rating
		FROM ratings
		ORDER BY student, course
		"
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())?;

	for rating in ratings {
		matrix.set(
			rating.student as usize - 1,
			rating.course as usize - 1,
			rating.rating as f32
		);
	}

	Ok(matrix)
}


pub async fn get_student_count(pool: &SqlitePool) -> Result<usize, String> {
	sqlx::query_scalar!(
		"
		SELECT COUNT(*) FROM students;
		"
	)
	.fetch_one(pool)
	.await
	.map_err(|e| e.to_string())
	.map(|res| res as usize)
}


pub async fn get_course_count(pool: &SqlitePool) -> Result<usize, String> {
	sqlx::query_scalar!(
		"
		SELECT COUNT(*) FROM courses;
		"
	)
	.fetch_one(pool)
	.await
	.map_err(|e| e.to_string())
	.map(|res| res as usize)
}


pub async fn get_students(pool: &SqlitePool) -> Result<Vec<Student>, String> {
	sqlx::query_as!(
		Student,
		"
		SELECT id, name
		FROM students;
		"
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())
}


pub async fn get_courses(pool: &SqlitePool) -> Result<Vec<Course>, String> {
	sqlx::query_as!(
		Course,
		"
		SELECT id, code, name, type
		FROM courses;
		"
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())
}


pub async fn get_types(pool: &SqlitePool) -> Result<Vec<String>, String> {
	sqlx::query!(
		"
		SELECT DISTINCT type
		FROM courses;
		"
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())
	.map(|v|
		v.iter()
		.map(|rec| rec.r#type.to_owned())
		.collect()
	)
}