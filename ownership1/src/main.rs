fn main() {
    let mut x = String::from("hi");
    let y = x;
    //x.push_str("hi");
    // ownership of x moved to y , so we are accessing a no longer valid variable
    // if we used x.clone() we could avoid this problem ,
    //because we will be duplicating the data and not just changing its reference
    //println!("{} , {}", x, y);
}
