// student* -> course* -> student*

struct Student<'a> {
    name: String,
    course: Vec<&'a Course<'a>>,
}

struct Course<'a> {
    name: String,
    students: Vec<&'a Student<'a>>,
}

impl<'a> Student<'a> {
    fn new(name: &str) -> Student<'a> {
        Student { name: name.into(), course: Vec::new() }
    }
}

impl<'a> Course<'a> {
    fn new(name: &str) -> Course<'a> {
        Course { name: name.into(), students: Vec::new() }
    }

    fn add_student(&'a mut self, student: &'a mut Student<'a>) {
        self.students.push(student);
        student.course.push(self);
        //  student is borrowed as mutable and immutable at the same time
        // RefCell is needed
    }
}