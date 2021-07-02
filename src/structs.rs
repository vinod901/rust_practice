// creating a struct
struct Student {
    name: String,
    id: String,
    course: String,
    number: i128,
    email: String,
}

// implementing functions for the struct
impl Student {
    fn introduce(&self) {
        println!(
            "Hello!, I am {}. I am studying {} at SSITS.",
            &self.name, &self.course
        );
    }
}

pub fn run() {
    let stud = Student {
        name: "vinod".to_string(),
        id: "18f71a0533".to_string(),
        course: "CSE".to_string(),
        number: 9014722319,
        email: "yerrapureddyvinodreddy@gmail.com".to_string(),
    };
    println!(
        "first admission :\nName : {}\nId : {}\nCourse : {}\nNumber : {}\nEmail : {}",
        stud.name, stud.id, stud.course, stud.number, stud.email
    );
    stud.introduce();
}
