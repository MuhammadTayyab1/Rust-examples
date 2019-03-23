use std::str;

fn main() {


    let mut count=65;
    let mut count1=97;
    for i in 0..26
    { 

    let cap = [count];
    let sam = [count1];
   

    let capital = String::from_utf8_lossy(&cap);
    let small = String::from_utf8_lossy(&sam);

    println!("{0} {1}", capital,small);
    count+=1;
    count1+=1;

    }


}
