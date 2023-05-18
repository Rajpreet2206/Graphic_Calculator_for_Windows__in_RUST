struct Rectangle{
    width: i32;
    height: i32;
}

impl Rectangle{
    pub fn new(width: i32, height: i32) -> Self{
        Self{width, height}
    }
}
