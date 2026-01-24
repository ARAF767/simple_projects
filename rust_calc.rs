use std::io::{self, Write};
#[derive(Default)]
#[derive(Clone)]
#[allow(non_snake_case)]
struct Courses{
    name: String,
    grade: f32,
    creditHour: u32,
}

#[allow(non_snake_case)]
fn input() -> String{
    let mut x = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut x)
        .expect("Something went wrong");
    return x;
}

#[allow(non_snake_case)]
fn unsigned32ToFloat32(i : u32) -> f32{
    let x : f32 = i as f32;
    return x;
}

#[allow(non_snake_case)]
fn calculateCGPA(data : Vec<Courses>) -> f32{
    let mut totalGradePoint : f32 = 0.0;
    let mut totalCreditHours : f32 = 0.0;
    
    for course in &data {
        totalGradePoint += course.grade * unsigned32ToFloat32(course.creditHour);
        totalCreditHours += unsigned32ToFloat32(course.creditHour);
    }
    if totalCreditHours > 0.0{
        return totalGradePoint/totalCreditHours;
    }else{
        return 0.0;
    }
}

#[allow(non_snake_case)]
#[allow(unused_assignments)]
fn main(){
    let mut courses: Vec<Courses> = Vec::new();

    print!("enter the number of courses : ");
    let str_numberOfCourse = input();
    let numberOfCourses : i32 = str_numberOfCourse.trim().parse().unwrap();
    let mut i :i32 = 0;
    while i < numberOfCourses{
        let mut course = Courses::default();
        println!("\n Enter the details for the course {}",i+1);
        
        print!("Course Name : ");
        course.name = input().trim().to_string();

        print!("Credit hour : ");
        course.creditHour = input().trim().parse().unwrap();

        print!("Grade (0-4) : ");
        course.grade = input().trim().parse().unwrap();
        if course.grade < 0.0 || course.grade > 4.0{
            println!("Invalid grade, Please enter a grade between 0  and 4");
            //i -= 1;
            continue;
        }
        courses.push(course);
        i += 1;
    }
    let cgpa = calculateCGPA(courses.clone());
    println!("\n Your CGPA is : {}",cgpa);
    for data in &courses{
        for _ in 0..20{
            print!("=");
        }
        println!(" ");
        println!("course name : {}", data.name);
        println!("Credit hour : {}", data.creditHour);
        println!("Grade       : {}", data.grade);
    }
}
