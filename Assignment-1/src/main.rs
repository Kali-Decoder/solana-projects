use std::fmt::Display;

trait Course {
    fn get_overview(&self) -> String;
}

impl Course for Workshop {
    fn get_overview(&self) -> String {
        format!(
            "Hey this is workshop of {}, instructor: {}, duration: {}",
            self.title, self.instructor, self.duration
        )
    }
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!(
            "Hey this is Seminar of {}, Speaker: {}, location: {}",
            self.title, self.speaker, self.location
        )
    }
}

#[derive(Debug)]
struct Workshop {
    title: String,
    instructor: String,
    duration: u16,
}
#[derive(Debug)]
struct Seminar {
    title: String,
    speaker: u16,
    location: String,
}

fn print_course_overview<T: Course>(course: T) {
    println!("{}",course.get_overview());
}

fn main() {
    let workshop = Workshop{
        title:"CSE".to_owned(),
        instructor:"Neeraj".to_owned(),
        duration:5
    };
    let seminar = Seminar{
        title:"CEC".to_owned(),
        speaker:3,
        location:"Titardi".to_owned()
    };
    print_course_overview(workshop);
    print_course_overview(seminar);
}
