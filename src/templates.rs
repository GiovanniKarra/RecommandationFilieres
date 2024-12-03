use askama_rocket::Template;

use crate::database::{Course, Student};


#[derive(Template)]
#[template(path = "full_matrix.html")]
pub struct RatingsMatrix {
	pub courses: Vec<Course>,
	pub students: Vec<Student>,
	pub matrix: Vec<Vec<f32>>
}

#[derive(Template)]
#[template(path = "courses_list.html")]
pub struct CoursesList {
	pub courses: Vec<Course>
}

#[derive(Template)]
#[template(path = "course_type_selection.html")]
pub struct TypeSelection {
	pub types: Vec<String>
}

#[derive(Template)]
#[template(path = "ratings_form.html")]
pub struct RatingsForm {
	pub courses: Vec<Course>
}