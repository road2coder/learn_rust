pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for cpn in self.components.iter() {
            cpn.draw();
        }
    }
}

// pub struct Screen<T>
// where
//     T: Draw,
// {
//     pub components: Vec<T>, // The components property of an instance can only contain one type.
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw a Button: {self:?}");
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw a SelectBox: {self:?}");
    }
}
