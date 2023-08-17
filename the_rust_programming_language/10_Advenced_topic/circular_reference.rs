// // student* -> course* -> student*

// struct Student<'a> {
//     name: String,
//     course: Vec<&'a Course<'a>>,
// }

// struct Course<'a> {
//     name: String,
//     students: Vec<&'a Student<'a>>,
// }

// impl<'a> Student<'a> {
//     fn new(name: &str) -> Student<'a> {
//         Student { name: name.into(), course: Vec::new() }
//     }
// }

// impl<'a> Course<'a> {
//     fn new(name: &str) -> Course<'a> {
//         Course { name: name.into(), students: Vec::new() }
//     }

//     fn add_student(&'a mut self, student: &'a mut Student<'a>) {
//         self.students.push(student);
//         student.course.push(self);
//         //  student is borrowed as mutable and immutable at the same time
//         // RefCell is needed
//     }
// }

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

    let john = Rc::new(RefCell::new(Student::new("John")));

    let course = Course::new("Rust course");
    let refcell_course = Rc::new(RefCell::new(course));

    Course::add_student(refcell_course, john);

    // for s in course.students {
    //     println!("student: {}", s.name);
    // }

}

