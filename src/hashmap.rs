use std::collections::HashMap;
pub fn run() {
    let mut a = HashMap::new();
    a.insert("vinod", "9014722319");
    a.insert("charan", "7799879538");
    a.insert("pavan", "9966024206");
    a.insert("ameen peer", "9505007137");
    a.insert("mithun", "9177088710");
    for (name, number) in a {
        println!("i am {} and my number is {}", name, number);
    }
}
