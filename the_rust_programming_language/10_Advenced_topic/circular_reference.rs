

use std::rc::Rc;
use std::cell::RefCell;

struct Student{
    name: String,
    course: Vec<Rc<RefCell<Course>>>,
}

struct Course{
    name: String,
    students: Vec<Rc<RefCell<Student>>>,
}

impl Student {
    fn new(name: &str) -> Student {
        Student { name: name.into(), course: Vec::new() }
    }
}

impl Course {
    fn new(name: &str) -> Course {
        Course { name: name.into(), students: Vec::new() }
    }

    fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
        student.borrow_mut().course.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

fn main(){
    // Rc gives .clone() method
    // RefCell gives .borrow() and .borrow_mut() methods
    let john = Rc::new(RefCell::new(Student::new("John")));

    let course = Course::new("Rust course");
    let refcell_course = Rc::new(RefCell::new(course));
    Course::add_student(refcell_course, john.clone());

    let course2 = Course::new("Rust course advanced");
    let refcell_course2 = Rc::new(RefCell::new(course2));
    Course::add_student(refcell_course2.clone(), john.clone());

    println!("john: {:?}", john.borrow().name);
    println!("john: {:?}", john.borrow().course[0].borrow().name);
    println!("john: {:?}", john.borrow().course[1].borrow().name);


    let jane = Rc::new(RefCell::new(Student::new("Jane")));

    Course::add_student(refcell_course2, jane.clone());
}


// database normalization
// Enrollments {course, student}

