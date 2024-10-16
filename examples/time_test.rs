use lazy_bastard::timer;
use lazy_bastard::timers::{end_timer, start_timer};

fn main() {
   let timer_start = start_timer("TEST");

   let mut v = 0;
   for i in 0..1_000_000_i128 {
      v += i.pow(4);
   }

   let elapsed = end_timer(timer_start);
   println!("{elapsed}");

   timer!(b, PRINT, {
      let mut v = 0;
      for i in 0..1_000_000_i128 {
         v += i.pow(4);
      }
   });

   timer!(test, PRINT, {
      let mut v = 0;
      for i in 0..1_000_000_i128 {
         v += i.pow(4);
      }
   });
}