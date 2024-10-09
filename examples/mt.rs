use std::marker::PhantomData;
use lazy_bastard::{lazy_bastard};

fn main() {
   let t = MyStruct::default();

   println!("{t:?}");
}


lazy_bastard!(
   #[derive(Clone, Debug)]
   pub struct MyStruct<'a> {
      field1: i32 => 100_324,
      field2: String => { "test".into() },
      field3: f64,

      phantom: PhantomData<&'a ()> => PhantomData::default(),

      long: f32 => {
         let c: f32 = 1.2;
         let v = c.abs().sin().sin().sqrt();
         0.1 * v
      }
   }
);