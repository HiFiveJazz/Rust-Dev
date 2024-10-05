struct Student {
    name: String,
    grade: Option<u32>,
}
fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db{
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(());

        }
    }
    Err(String::from("Student not found"));
}

fn main () {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(96),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(84), 
        },
        Student {
            name: String::from("Charlie"),
            grade: None, 
        },
    ];
    let student_name = String::from("Charlie");
    let student_status = check_student(&student_name, &student_db)

    match student_status {
        Ok(_) {
            let student_grade = get_grade(&student_name, &student_db);
            if let Some(grade) = student_grade {
                println!("Grade is: {grade}");
            }
        }
    }

    match student_grade {
        Some(grade) => println!("Grade is: {grade}"),
        None => println!("No Grade Present")
    }
}
