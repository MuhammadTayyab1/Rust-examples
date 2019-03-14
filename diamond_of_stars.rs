
fn main() {
        
         let mut i=0;
         let mut count=10;
         loop
         {
            if i < 10
            {
              i+=1;
               for x in 0..count
               {
                   print!("*");
               }
               count-=1;
               for y in 0..2*i
               {
                    print!(" ");
               }
               for z in 0..count
               {
                   print!("*");
               }
               println!("");
            }
            else
            {
                break;
            }
         }


          let mut j=11;
         loop
         {
            if j > 1
            {
              j-=1;
               for x in 0..count
               {
                print!("*");
               }
               count+=1;
               for y in 0..2*j
               {
                print!(" ");
               }
               for z in 0..count
               {
                print!("*");
               }
               println!("");
            }
            else
            {
                break;
            }
         }
}
