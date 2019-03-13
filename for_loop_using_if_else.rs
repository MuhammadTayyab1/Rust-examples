fn main() {

     let mut a=0;
    
     loop 
      {
           if a <= 100
            {
              println!("Print value : {0} ",a);
              a+=1;
              continue;
            }
            else
            {
               break;
            }
      }
}
