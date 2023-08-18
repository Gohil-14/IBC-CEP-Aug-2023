use std::io;

// Define the Employee struct
struct Employee {
    employee_name: String,
    employee_id: u32,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    // Constructor for Employee
    fn new(name: String, id: u32, email: String, age: u32, phone: String) -> Self {
        Employee {
            employee_name: name,
            employee_id: id,
            email,
            age,
            phone_number: phone,
        }
    }
}

// Function to find an employee by employee_id
fn find_employee_by_id(employees: &[Employee], id: u32) -> Option<&Employee> {
    employees.iter().find(|&emp| emp.employee_id == id)
}

// Function to find employees with the same age
fn find_employees_by_age(employees: &[Employee], target_age: u32) -> Vec<&Employee> {
    employees.iter().filter(|&emp| emp.age == target_age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("Enter employee details:");
        
        println!("Employee Name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();
        
        println!("Employee ID:");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read line");
        let id: u32 = id.trim().parse().expect("Please enter a valid number");
        
        println!("Email:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");
        let email = email.trim().to_string();
        
        println!("Age:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read line");
        let age: u32 = age.trim().parse().expect("Please enter a valid number");
        
        println!("Phone Number:");
        let mut phone = String::new();
        io::stdin().read_line(&mut phone).expect("Failed to read line");
        let phone = phone.trim().to_string();
        
        let employee = Employee::new(name, id, email, age, phone);
        employees.push(employee);

        println!("Do you want to add another employee? (y/n)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        if choice.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Enter an employee ID to search:");
    let mut search_id = String::new();
    io::stdin().read_line(&mut search_id).expect("Failed to read line");
    let search_id: u32 = search_id.trim().parse().expect("Please enter a valid number");

    if let Some(found_employee) = find_employee_by_id(&employees, search_id) {
        println!("Found Employee by ID:");
        println!("Name: {}", found_employee.employee_name);
        println!("Email: {}", found_employee.email);
        println!("Age: {}", found_employee.age);
        println!("Phone: {}", found_employee.phone_number);
    } else {
        println!("Employee not found with the given ID.");
    }

    println!("Enter an age to search for employees with that age:");
    let mut search_age = String::new();
    io::stdin().read_line(&mut search_age).expect("Failed to read line");
    let search_age: u32 = search_age.trim().parse().expect("Please enter a valid number");

    let employees_with_same_age = find_employees_by_age(&employees, search_age);
    if employees_with_same_age.is_empty() {
        println!("No employees found with the given age.");
    } else {
        println!("Employees with the same age:");
        for emp in employees_with_same_age {
            println!("Name: {}", emp.employee_name);
            println!("Email: {}", emp.email);
            println!("Age: {}", emp.age);
            println!("Phone: {}", emp.phone_number);
            println!("---");
        }
    }
}
