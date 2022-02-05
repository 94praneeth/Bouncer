use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: u8,
    greeting: String,
}
impl Visitor {
    fn new(name: &str, action: VisitorAction, age: u8, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("{}", self.greeting),
            VisitorAction::AcceptWithNote { note } => {
                println!("{}", self.greeting);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alchol to {}.\n", self.name)
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member.\n", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in.\n", self.name),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read input");

    your_name.trim().to_lowercase()
}
fn what_is_your_age() -> u8 {
    let mut age = String::new();
    stdin().read_line(&mut age).expect("Failed to read input");

    if age.is_empty() {
        age = "0".to_string();
    }
    match age.trim().parse() {
        Ok(v) => v,
        _ => 0,
    }
}
fn main() {
    let mut visitor_list = vec![
        Visitor::new(
            "bert",
            VisitorAction::Accept,
            45,
            "Hi Bert! Welcome to the club.\n",
        ),
        Visitor::new(
            "frank",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
            "Hello Frank! Your phone is in the shelf.\n",
        ),
        Visitor::new(
            "fred",
            VisitorAction::Refuse,
            30,
            "Wow! Who invited Fred?\n",
        ),
    ];
    loop {
        println!("Hello, What is your name? (Leave Empty and press ENTER to quit)\n");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(v) => v.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!(
                        "{} is not on the list.\n--> Adding {0} to the visitor list.\n",
                        name
                    );
                    println!("What is your age {}? (Please Enter a number for age or Leave empty and Press ENTER to quit)", name);
                    let age = what_is_your_age();

                    if age == 0 {
                        break;
                    } else if age < 21 {
                        visitor_list.push(Visitor::new(
                            &name,
                            VisitorAction::AcceptWithNote {
                                note: String::from("Non alcholic beverages only."),
                            },
                            age,
                            &format!("Hi new friend {}.", name),
                        ));
                    } else {
                        visitor_list.push(Visitor::new(
                            &name,
                            VisitorAction::Refuse,
                            age,
                            &format!("You are not allowed in {}.\n", name),
                        ));
                    }
                }
            }
        }
    }
    // println!("{:#?}", visitor_list);
}
