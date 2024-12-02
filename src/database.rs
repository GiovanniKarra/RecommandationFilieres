use std::path::PathBuf;

use std::fs::File;
use sqlx::SqlitePool;

use libmatcompl::FMatrix;


#[derive(Debug)]
struct Student {
	id: i64,
	name: String
}

#[derive(Debug)]
struct Course {
	id: i64,
	code: String,
	name: String
}

#[derive(Debug)]
pub struct Rating {
	id: i64,
	student: i64,
	course: i64,
	rating: i64
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

	if student.is_some() { return Err("Student with this name already exists".to_owned()) }
	
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


pub async fn add_course(name: String, code: String, pool: &SqlitePool) -> Result<(), String> {
	let course = sqlx::query_as!(
		Course,
		"
		SELECT id, code, name FROM courses
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
		INSERT INTO courses (code, name)
		VALUES (?, ?);
		",
		code, name
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


pub async fn get_student_ratings(student_id: i64, pool: &SqlitePool) -> Result<Vec<Rating>, String> {
	sqlx::query_as!(
		Rating,
		"
		SELECT id, student, course, rating FROM ratings
		WHERE student = ?;
		",
		student_id
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())
}


pub async fn get_ratings_matrix(pool: &SqlitePool) -> Result<FMatrix, String> {
	let m = get_student_count(pool).await?;
	let n = get_course_count(pool).await?;

	let data = sqlx::query_scalar!(
		"
		SELECT rating
		FROM ratings
		ORDER BY student, course
		"
	)
	.fetch_all(pool)
	.await
	.map_err(|e| e.to_string())?
	.iter()
	.map(|elem| *elem as f32)
	.collect();


	Ok(FMatrix::new(m, n, data))
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