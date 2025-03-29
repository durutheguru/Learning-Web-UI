
pub trait Area {
    fn area(&self) -> f32;
}

pub struct Circle {
    pub radius: f32,
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}


impl Area for Circle {

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

}


impl Area for Rectangle {
    
    fn area(&self) -> f32 {
        self.width * self.height
    }

}
