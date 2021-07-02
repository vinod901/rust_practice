pub fn run() {
    // creating a vector that stores string data
    let mut stack: Vec<&str> = vec!["vinod", "pavan", "charan"];

    // pushing data into the vector at the end
    stack.push("ameen peer");
    stack.push("some crap...!");

    // printing the vector elements
    println!("Elements of the stack are {:?}", stack);

    // popping elements from the vector
    stack.pop();
    println!("Elements of the stack after popping once are {:?}", stack);

    // printing elements of the vector one after the other
    for name in stack.iter() {
        println!("{} is in stack", name);
    }

    // finding the length of the vector
    println!("Lenght of the stack is {}", stack.len());
}
