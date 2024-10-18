//! inline defaults for struct definitions if you're lazy, plus some other stuff, probably.
//!
//! makes creating simple structs that don't need a constructor like settings structs much clearer
//!
//! somewhat second guessing the name, but it's too late to change it
//!
//! ```
//! use lazy_bastard::lazy_bastard;
//!
//! lazy_bastard!(
//!    #[derive(Clone, Debug)]
//!    pub struct MyStruct<'a> { // lifetime is just for the example
//!       normal: i32 => 100_324,
//!       function_call: String => "test".into(),
//!       automatic: f64, // uses Default::default() instead
//!       scoped: f32 => {
//!          let c: f32 = 1.2;
//!          let v = c.abs().sin().sin().sqrt();
//!          0.1 * v
//!       }
//!    }
//! );
//!
//!
//! impl MyStruct {
//!     fn new() -> Self { Self::default() }
//! }
//!
//!
//! // Compiles into //
//!
//! #[derive(Clone, Debug)]
//! pub struct CompiledStruct<'a> {
//!    normal: i32,
//!    function_call: String,
//!    automatic: f64,
//!    scoped: f32,
//! }
//! impl<'a> Default for CompiledStruct<'a> {
//!    fn default() -> Self {
//!       Self {
//!          normal: 100_324,
//!          function_call: ("test".into()),
//!          automatic: Default::default(),
//!          scoped: {
//!             let c: f32 = 1.2;
//!             let v = c.abs().sin().sin().sqrt();
//!             0.1 * v
//!          },
//!       }
//!    }
//! }
//!
//! ```

pub mod macros;
// pub use timers;
// pub use globals::*;


pub mod timers {
   use std::fmt::{Display, Formatter};
   use std::time::{Duration, Instant};

   #[derive(Debug, Clone, Copy)]
   pub struct CurrentTimer {
      pub st: Instant,
      pub name: &'static str
   }

   #[derive(Debug, Clone, Copy)]
   pub struct TimedSection {
      pub elapsed: Duration,
      pub name: &'static str,
   }

   impl Display for TimedSection {
      fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         write!(f, "{} <{:?}>", self.name, self.elapsed)
      }
   }

   pub fn start_timer(name: &'static str) -> CurrentTimer {
      CurrentTimer {
         st: Instant::now(),
         name,
      }
   }

   pub fn end_timer(current_timer: CurrentTimer) -> TimedSection {
      TimedSection {
         elapsed: current_timer.st.elapsed(),
         name: current_timer.name,
      }
   }
}

pub mod globals {
   use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

   pub fn safe_get_mut<T, F>(name: &'static Arc<RwLock<T>>, code: F)
   where
       F: FnOnce(RwLockWriteGuard<'static, T>),
   {
      code(name.write().unwrap());
   }

   pub fn safe_get_ref<T, F>(name: &'static Arc<RwLock<T>>, code: F)
   where
       F: FnOnce(RwLockReadGuard<'static, T>),
   {
      code(name.read().unwrap());
   }
}
