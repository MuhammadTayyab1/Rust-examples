fn main() {
  let mut a=87;
  let mut b=90;
  
  println!("Origanal values");
  println!("a = {0}   b = {1}",a,b);

  let mut hold=0;
  hold = a;
  a= b;
  b = hold;
 
  println!("Swaped values");
  println!("a = {0}   b = {1}",a,b);
}
