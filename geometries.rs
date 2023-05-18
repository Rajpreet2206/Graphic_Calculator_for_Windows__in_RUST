struct Rectangle{
    width: i32;
    height: i32;
}

impl Rectangle{
    pub fn new(width: i32, height: i32) -> Self{
        Self{width, height}
    }
}

struct Square{
    length: i32;
}

impl Square{
    pub fn new(length: i32) -> Self{
        Self(length)
    }
    pub fn get_length(&self) -> i32{
        self.length
    }
}