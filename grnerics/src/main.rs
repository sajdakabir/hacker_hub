
#[derive(Debug)]
struct Point <T, U>{
    x: T,
    y: U
}
impl <T, U> Point<T, U>{
    fn new(x: T, y: U)-> Self{
        Self { x, y }
    }

    fn mixup<X, Y> (self, point : Point< X, Y >)-> Point<T, Y>{
        Point{
           x: self.x,
            y: point.y
        }
    }
}

// impl  Point<f64, f64> {
//     fn distance_from_origin(&self)-> f64 {
//         9.8
//     }
// }
fn main() {

    let list = vec![1,34,5,2,3,4];
    let list1 = vec![1.4,3.4,5.4,2.5,4.0];
    let r= get_largest_num(&list);
    let r1= get_largest_num(&list1);

    println!("the value is {r}");
    println!("the value is {r1}");

    let integer = Point{
        x: 3,
        y: 4.4
    };
    
    let flote = Point{
        x: 3.9,
        y: 4.0
    };
    // flote.distance_from_origin();
    
    let point = Point::new(1, 2);
    let p3= integer.mixup(flote);
    println!("the vaule is {p3:?}");
}


fn get_largest_num<T:  std::cmp::PartialOrd>(list: &[T])-> &T {
    let mut ans = &list[0];
    for i in list {
        if ans<i {
            ans=i;
        }
    }
    ans
}
