use std::collections::HashMap;
pub fn run() {
    // creating a hashmap
    let mut dict = HashMap::new();

    // adding key value pairs to the hashmap
    dict.insert("vinod", "9014722319");
    dict.insert("charan", "7799879538");
    dict.insert("pavan", "9966024206");
    dict.insert("ameen peer", "9505007137");
    dict.insert("mithun", "9177088710");

    // accessing key value pairs from the hashmap
    for (name, number) in dict {
        println!("i am {} and my number is {}", name, number);
    }
}
