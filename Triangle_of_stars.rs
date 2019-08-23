
fn main() {
        
         let mut i=0;
        // for one side
         loop
         {
            if i < 20
            {
              i+=2;
               for x in 0..i
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

        
        
          // for another side
          let mut j=20;
         loop
         {
            if j > 0
            {
              j-=2;
               for x in 0..j
               {
                print!("*");
                continue;
               }
               println!("");
            }
            else
            {
                break;
            }
         }
}
