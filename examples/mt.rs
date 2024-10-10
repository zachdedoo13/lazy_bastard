use lazy_bastard::{lazy_bastard, time_section_print};

fn main() {
   let t = MyStruct::default();

   time_section_print!("Section", {
      let mut c = 87;
   });

   c += 1;

   println!("{t:?}");
}


lazy_bastard!(
   #[derive(Clone, Debug)]
   pub struct MyStruct {
      normal: i32 => 100_324,
      function_call: String => "test".into(),
      automatic: f64, // uses Default::default() instead
      scoped: f32 => {
         let c: f32 = 1.2;
         let v = c.abs().sin().sin().sqrt();
         0.1 * v
      }
   }
);