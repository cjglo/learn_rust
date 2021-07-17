

pub trait Draw {
    fn drawe(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {  // * We could do Vec<T> with where T: draw ... but then Vec would have to be 1 type, not any type that has draw trait
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


fn main() {

}