

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self)-> u32{
        self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
         self.width>=other.width && self.height>=other.height 
    }

    fn squ(width:u32)-> Self{
        Self{
            width:width,
            height:width
        }
    }
}




fn main() {

    let rect = Rectangle{
        width:20,
        height: 20
    };

    let rect1 = Rectangle {
        width: 90,
        height: 90
        
    };

    let s= Rectangle::squ(20);

    println!("the area of the rectangle is {}", rect.area());

    println!("can rect hold rect1 {}", rect.can_hold(&rect1));
    println!("can rect1 hold rect {}", rect1.can_hold(&rect));

    println!("the value of s is {:?}", s)


   
}
