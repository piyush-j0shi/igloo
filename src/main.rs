use std::{collections::HashMap, hash::Hash, ops::Sub};

#[derive(Debug)]
enum GradingScale {
    Percentage,
    LetterGrade,
}

#[derive(Debug)]
struct Subject {
    grades: Vec<f64>,
    weight: f64,
    grading_scale: GradingScale,
}

#[derive(Debug)]
struct Student {
    name: String,
    subjects: HashMap<String, Subject>,
}

#[derive(Debug)]
struct StudentDatabase {
    students: HashMap<String, Student>,
}

impl StudentDatabase {
    fn new() -> Self {
        let studentdbnew: HashMap<String, Student> = HashMap::new();
        Self {
            students: studentdbnew,
        }
    }

    fn add_student(&mut self, name: &str) {
        let english = Subject {
            grades: vec![89.0, 92.0, 39.0, 42.0],
            weight: 1.0,
            grading_scale: GradingScale::Percentage,
        };

        let mut subject_map: HashMap<String, Subject> = HashMap::new();
        subject_map.insert("english".to_string(), english);

        let student = Student {
            name: name.to_string(),
            subjects: subject_map,
        };

        self.students.insert(name.to_string(), student);
    }

    fn calculate_grade(&mut self, student_name: &str, subject_name: &str) {
        if let Some(student_details) = self.students.get(&student_name.to_string()) {
            if let Some(subject) = student_details.subjects.get(&subject_name.to_string()) {
                println!("student_details : {:?}", subject.grades);

                let mut total_marks = 0.0;
                let total = subject.grades.len() as f64;

                for marks in &subject.grades {
                    total_marks += marks;
                }
                println!("average is : {}", total_marks / total);
            } else {
                println!("nothing found");
            }
        } else {
            println!("student does not exists");
        }
    }
}

fn main() {
    let mut studentdb = StudentDatabase::new();
    studentdb.add_student("student1");
    studentdb.add_student("student2");
    studentdb.add_student("student3");

    println!("new student : {:#?}", studentdb);

    studentdb.calculate_grade("student1", "english");
}
