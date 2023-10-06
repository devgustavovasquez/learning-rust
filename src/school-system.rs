use std::io;

struct StudentReport {
    id: u32,
    name: String,
    n1: f32,
    n2: f32,
    n3: f32,
    average: Option<f32>,
    is_pass: Option<bool>,
}

fn clear_prompt() {
    println!("\x1B[2J\x1B[1;1H");
}

fn calc_average(n1: f32, n2: f32, n3: f32) -> f32 {
    (n1 + n2 + n3) / 3.0
}

fn calc_is_pass(student: &mut StudentReport) {
    let average: f32 = calc_average(student.n1, student.n2, student.n3);
    let is_pass: bool = average >= 7.0;
    student.average = Some(average);
    student.is_pass = Some(is_pass);
}

// calc avarage and is pass when add new student
fn add_new_student(students: &mut Vec<StudentReport>, student: StudentReport) {
    let mut student = student;
    calc_is_pass(&mut student);
    students.push(student);
}

fn list_students(students: &mut Vec<StudentReport>) {
    clear_prompt();
    
    for student in students {
        println!("ID: {}", student.id);
        println!("Name: {}", student.name);
        println!("Note 1: {}", student.n1);
        println!("Note 2: {}", student.n2);
        println!("Note 3: {}", student.n3);
        println!("Average: {}", student.average.unwrap());
        println!("Is pass: {}", student.is_pass.unwrap());
        println!("------------------------");
    }
}

fn prompt_new_student(students: &mut Vec<StudentReport>) {
    println!("Add new student");
    println!("------------------------");
    println!("ID: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id: u32 = id.trim().parse().expect("Please type a number");

    println!("Name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name: String = name.trim().parse().expect("Please type a string");

    println!("Note 1: ");
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1).expect("Failed to read line");
    let n1: f32 = n1.trim().parse().expect("Please type a number");

    println!("Note 2: ");
    let mut n2 = String::new();
    io::stdin().read_line(&mut n2).expect("Failed to read line");
    let n2: f32 = n2.trim().parse().expect("Please type a number");

    println!("Note 3: ");
    let mut n3 = String::new();
    io::stdin().read_line(&mut n3).expect("Failed to read line");
    let n3: f32 = n3.trim().parse().expect("Please type a number");

    let student = StudentReport {
        id,
        name,
        n1,
        n2,
        n3,
        average: None,
        is_pass: None,
    };

    add_new_student(students, student);
    clear_prompt();
}


fn menu() {
    println!("------------------------");
    println!("School System");
    println!("1 - Add new student");
    println!("2 - List students");
    println!("3 - Exit");
    println!("------------------------");
    println!("Choose an option: ");
}
fn main() {
    let mut students: Vec<StudentReport> = Vec::new();
    clear_prompt();
    loop {
        menu();
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = option.trim().parse().expect("Please type a number");

        match option {
            1 => prompt_new_student(&mut students),
            2 => list_students(&mut students),
            3 => {
                clear_prompt();
                break;
            }
            _ => println!("Invalid option"),
        }
    }    
}
