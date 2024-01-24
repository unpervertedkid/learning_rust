use std::rc::Rc;

struct Owner {
    name: String, 
    tools: Vec<Tool>,
}

struct Tool {
    name: String,
    owner: Rc<Owner>,
}

fn main() {
    let brian = Rc::new(Owner { name : String::from("Brian"), tools: Vec::new() });

    let pliers = Tool { name: String::from("pliers"), owner: Rc::clone(&brian)};
    let hammer = Tool { name: String::from("hammer"), owner: Rc::clone(&brian)};
    
}
