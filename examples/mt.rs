use lazy_bastard::{lazy_bastard, time_section_print};

fn main() {
   let t = MyStruct::default();

   time_section_print!("Section", {
      let mut c = 87;
   });

   c += 1;

   println!("{c}");

   println!("{t:?}");
}

const TEST: f64 = 0.0;

lazy_bastard!(
   #[derive(Clone, Debug)]
   pub struct MyStruct<'a> {
      pub normal: i32 => 100_324,
      function_call: String => "test".into(),
      automatic: f64, // uses Default::default() instead
      lifetime: &'a f64 => &TEST,
      scoped: f32 => {
         let c: f32 = 1.2;
         let v = c.abs().sin().sin().sqrt();
         0.1 * v
      }
   }
);

lazy_bastard!(
   pub struct TT {
      one: i32 = 0,
      two: i32 = 0,
   }
);