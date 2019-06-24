
fn main() {
        
         println!("Array of numbers");
         println!("=========================================");
         let mut numdata=[100,90,87,88,95,95,99,100,150,100,92];
 
         for i in 0..9
         {

            println!("{0}",numdata[i]);

         }
        
        
         println!("=========================================");
         println!(" ");
         println!("Array of text");
         println!("=========================================");
        
        // initilize array and assign data 
         let mut textdata=["text","my text","own data","rust","webassembly","country","computer","cargo run","function","start"];

         for j in 0..textdata.len()
         {

            println!("{0}",textdata[j]);
         }
         println!("=========================================");
       

         
}
