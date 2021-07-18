

pub trait Draw {
    fn draw(&self);
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

// * Now lets define some types to use the Draw trait with

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("DRAWING BUTTON>>>");
    }
}

struct SelectBox {
    pub side: u32,
    pub isChecked: bool,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("DRAWING SELECTBOX!");
    }
}



fn main() {

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                side: 70,
                isChecked: false,
            }),
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    screen.run();
}