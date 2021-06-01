// * This continues the lesson, but with a tree data structure to show off Weak<T> and some other 


use std::rc::Rc;
use std::cell:RefCell;

#[derive(Debug)]
struct Node {
    value: i32, 
    children: RefCell<Vec<Rc<Node>>>
}
// * ^ we create a Node that knows of its children 

fn example() {
    let leaf = Rc::new(Node { value:3, children: RefCell::new(vec![]) });

    let branch = Rc::new(Node { value:value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    })

    // Notice: Can get from branch -> leaf but not back.  Parent owns child, but child doesn't ref parent

    // We can use a weak reference here, cause it makes sense to have parent own a child (and so drop it if it is dropped), but
        // not have child own Parent (keep parent even if child is dropped)

    // We need weak cause otherwise circular reference would exist 

    


}