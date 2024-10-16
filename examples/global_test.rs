use lazy_bastard::{init_global_mut, safe_get_mut};

init_global_mut!(DATA: Vec<f32> => vec![]);

type MD = Vec<f32>;

fn main() {
   println!("Original state => {:?}", DATA.read().unwrap());
   tfn(&mut DATA.write().unwrap());

   let one = std::thread::spawn(|| {
      safe_get_mut(&DATA, |mut d| {
         tfn(&mut d);
      });

   });

   one.join().unwrap();

   println!("End state => {:?}", DATA.read().unwrap());
}

fn tfn(data: &mut MD) {
   let zero: f32 = 0.0;
   let st = *data.last().unwrap_or(&zero) as usize;
   let et = st + 10;

   for i in st..et {
      data.push(i as f32);
   }
}