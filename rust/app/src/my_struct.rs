
pub struct MyStruct {
    pub field1: i32,
    pub field2: String,
}


impl MyStruct {

    pub fn new(field1: i32, field2: String) -> MyStruct {
        MyStruct { field1, field2, }
    }

    pub fn display(&self) {
        println!(">>>Field1: {}, Field2: {}", self.field1, self.field2);
    }

}

