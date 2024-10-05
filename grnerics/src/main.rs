fn main() {

    let list = vec![1,34,5,2,3,4];
    let r= get_largest_num(&list);

    println!("the value is {r}");
    
}

fn get_largest_num(list: &[i32])-> &i32 {
    let mut ans = &list[0];
    for i in list {
        if ans<i {
            ans=i;
        }
    }
    ans
}
