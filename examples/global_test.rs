use std::cell::RefCell;
use std::rc::Rc;
use lazy_bastard::{init_global_mut, globals::safe_get_mut};


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


#[test]
fn tt() {
   // Definition for a binary tree node.
   #[derive(Debug, PartialEq, Eq)]
   pub struct TreeNode {
     pub val: i32,
     pub left: Option<Rc<RefCell<TreeNode>>>,
     pub right: Option<Rc<RefCell<TreeNode>>>,
   }

   impl TreeNode {
     #[inline]
     pub fn new(val: i32) -> Self {
       TreeNode {
         val,
         left: None,
         right: None
       }
     }
   }

   type NodeCon = Rc<RefCell<TreeNode>>;

   use std::rc::Rc;
   use std::cell::RefCell;
   // impl Solution {
   pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let mut max = 0;

      dfs(&root.unwrap(), &mut max);

      max
   }
   // }


   fn dfs(node: &NodeCon, max: &mut i32) -> i32 {
      let mut local_max = access_data(node).val;

      if let Some(left) = &access_data(node).left {
         local_max += dfs(&left, max);
      }

      if let Some(right) = &access_data(node).right {
         local_max += dfs(&right, max);
      }

      if local_max > *max {
         *max = local_max;
      }

      local_max
   }

   fn access_data<T>(rc_refcell: &Rc<RefCell<T>>) -> std::cell::Ref<T> {
      rc_refcell.borrow()
   }

   fn access_data_mut<T>(rc_refcell: &Rc<RefCell<T>>) -> std::cell::RefMut<T> {
      rc_refcell.borrow_mut()
   }
}

