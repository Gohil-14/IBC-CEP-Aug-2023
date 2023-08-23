struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student {
        name: String::from("John Doe"),
        email: String::from("john@example.com"),
        phno: String::from("123-456-7890"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Jane Smith"),
        email: String::from("jane@example.com"),
        phno: String::from("987-654-3210"),
        id: 2,
    });

    // Add more students as needed

    let student_index = 1; // Change this to the index of the student you want to access

    match students.get(student_index) {
        Some(student) => {
            println!("Student Name: {}", student.name);
            println!("Student Email: {}", student.email);
            println!("Student Phone Number: {}", student.phno);
            println!("Student ID: {}", student.id);
        }
        None => {
            eprintln!("Student not found at index {}", student_index);
        }
    }
}
