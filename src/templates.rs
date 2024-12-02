use askama_rocket::Template;

use crate::database::{Course, Student};


#[derive(Template)]
#[template(path = "full_matrix.html")]
pub struct RatingsMatrix {
	pub courses: Vec<Course>,
	pub students: Vec<Student>,
	pub matrix: Vec<Vec<f32>>
}