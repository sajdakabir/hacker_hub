
struct Point <T>{
    x: T,
    y: T
}

fn main() {

    let list = vec![1,34,5,2,3,4];
    let list1 = vec![1.4,3.4,5.4,2.5,4.0];
    let r= get_largest_num(&list);
    let r1= get_largest_num(&list1);

    println!("the value is {r}");
    println!("the value is {r1}");

    let integer = Point{
        x: 3,
        y: 4
    };
    
    let flote = Point{
        x: 3.9,
        y: 4.0
    };
    
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
