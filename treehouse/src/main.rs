use std::io::stdin;

fn main() {

    let mut visitor_list = vec! [
        Visitor::new("bert",
                     "Hello, Bert, enjoy your treehouse.",
                     VisitorAction::Accept {note : "".to_string()},
                     45),
        Visitor::new("steve",
                     "Hi Steve. Your milk is in the fridge.",
                     VisitorAction::Accept {note: String::from("Lactose-free milk is in the fridge")},
                     15),
        Visitor::new("fred", "Wow, who invited Fred?", VisitorAction::Refuse, 30)
    ];

    loop {
        println!("Hello, world!");
        let name = hello_yourname();
        treehouse_guestlist(&name);

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None          =>
                if name.is_empty() {
                    break;
                } else {
                    println!("{} weren't invited, adding to list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend", VisitorAction::Probation, 21));
                }

        }
    }

    println!("Final list of invitees:\n {:#?}", visitor_list);
}

fn hello_yourname() -> String {
    println!("Hello, what's your name?");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    return your_name
        .trim()
        .to_lowercase();
}

fn treehouse_guestlist(name : &str){
    let visitor_list = ["a", "b", "c"];
    let mut allow_them_in = false;
    for visitor in visitor_list {
        if visitor == name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome to Treehouse, {}", name);
    }
    else {
        println!("Sorry, you aren't on the list.");
    }
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8
}

#[derive(Debug)]
enum VisitorAction {
    Accept {note : String},
    Refuse,
    Probation
}


impl Visitor {
    fn new(name : &str, greeting : &str, action : VisitorAction, age : i8) -> Self {
        Self {
            name : name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);

        match &self.action {
            VisitorAction::Accept { note } => {
                println!("Welcome to the tree house, {}.", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}
