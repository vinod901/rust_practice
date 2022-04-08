use joinable::Joinable;

extern crate joinable;

#[derive(Debug)]
struct Person {
    id: u64,
    name: String,
}

#[derive(Debug)]
struct Mobile {
    id: u64,
    number: String,
}
#[get("/sql/joins")]
pub fn joins() -> String {
    let friends = [
        Person {
            id: 1,
            name: "Vinod".to_string(),
        },
        Person {
            id: 2,
            name: "Mithun".to_string(),
        },
        Person {
            id: 3,
            name: "Rishi".to_string(),
        },
        Person {
            id: 4,
            name: "Prasanth".to_string(),
        },
        Person {
            id: 5,
            name: "Charan".to_string(),
        },
        Person {
            id: 6,
            name: "Pavan".to_string(),
        },
    ];
    let mobile = [
        Mobile {
            id: 1,
            number: "9014722319".to_string(),
        },
        Mobile {
            id: 1,
            number: "9177088710".to_string(),
        },
        Mobile {
            id: 1,
            number: "6305519510".to_string(),
        },
        Mobile {
            id: 1,
            number: "9100677616".to_string(),
        },
        Mobile {
            id: 1,
            number: "7780757791".to_string(),
        },
        Mobile {
            id: 1,
            number: "9966024206".to_string(),
        },
    ];
    let join = friends
        .iter()
        .outer_join(&mobile[..], |f, m| f.id.cmp(&m.id));
    for (person, mobile) in join {
        println!("name : {}, number : {:?}", person.name, mobile);
    }
    format!("Success")
}
