// Define the `User` struct
#[derive(Clone)] // Automatically derive the `Clone` trait for `User`
pub struct User {
    name: String,
    age: u32,
}
// impl block : for implementing struct methods
impl User {
    pub fn new(name: String, age: u32) -> Self {
        User { name, age }
    }
    fn transform(self) -> User {
        User {
            name: String::from("transformed"),
            age: self.age,
        }
    }
    fn pretty(&mut self) -> String {
        //self.name = String::from("pretty name");
        format!("{} {}", self.name, self.age)
    }
}

// The main function
fn main() {
    // Initialize `user1` with name and age
    let mut user1 = User {
        name: String::from("mehdi"),
        age: 10,
    };

    // Clone `user1` to create `user2`
    let user2 = user1.clone(); // `clone` is available because of `#[derive(Clone)]`
    user1.name = String::from("kasmi");
    // transform took the ownership and gave it back to user1 ,
    user1 = user1.transform();
    // if we instead done this :
    // let user3 = user1.transform()
    // user1 will no longer be vali after this valid because it lost its ownership
    // Print the name of `user1` and `user2`
    println!("user1: {}, user2: {}", user1.pretty(), user2.name);
    user1.name = String::from("hi");
}
