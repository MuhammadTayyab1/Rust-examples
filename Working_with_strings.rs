fn main() {

    println!("Real form ");
    println!("hello\n");
    let s = String::from("hello");
    let slice = &s[0..5];
    println!("slice form \n 0-5 \n {0}  \n\n",slice);



    println!("Real form ");
    println!("hello world \n");
    let q = String::from("hello world");
    let slice1= &q[2..8];
    println!("slice form \n 2-8 \n {0} \n\n ",slice1);

    println!("String joining ");
    let mut s = String::from("hello");
    s.push_str(", world!"); 
    println!("{} \n\n", s);

   
    println!("String cloning");
    let q1 = String::from("hello");
    let q2 = q1.clone();
    println!("s1 = {}, s2 = {}", q1, q2);
    

}


