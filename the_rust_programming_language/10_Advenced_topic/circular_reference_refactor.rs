struct Student {
    name: String,
}

impl Student {
    fn courses(&self, platform: &Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course {
    name: String,
}

impl Course {
    fn new(name: &str) -> Course {
        Course { name: name.into() }
    }
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment <'a>{
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform <'a> {
    fn new() -> Platform<'a> {
        Platform { enrollments: Vec::new() }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}


impl Student {
    fn new(name: &str) -> Student {
        Student { name: name.into() }
    }
}


fn main(){

    let john = Student::new("John");
    let course = Course::new("Rust course");

    let mut platform = Platform::new();
    platform.enroll(&john, &course);

    println!("john: {:?}", john.name);
    println!("course: {:?}", course.name);


    for c in john.courses(&platform) {
        println!("course: {:?}", c);
    }
}
