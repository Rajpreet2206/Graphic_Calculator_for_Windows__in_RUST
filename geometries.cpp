class Rectangle{
    protected:
        int width;
        int height;
    
    public:
        Rectangle(int width, int height) : width(width), height(height){}
        int get_area(){return width*height;}
        int get_width(){return height;}
};

class Square: public Rectangle{
    public:
        Square(int length) : Rectangle(length, length){}
        int get_length(){return width;}
};
