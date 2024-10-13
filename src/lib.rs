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

/// inline defaults for struct definitions if your lazy
/// ```
/// use lazy_bastard::lazy_bastard;
///
/// lazy_bastard!(
///    #[derive(Clone, Debug)]
///    pub struct MyStruct<'a> {
///       field1: i32 => 100_324,
///       field2: String => { "test".into() },
///       field3: f64,
///
///       long: f32 => {
///          let c: f32 = 1.2;
///          let v = c.abs().sin().sin().sqrt();
///          0.1 * v
///       }
///    }
/// );
/// ```
#[macro_export]
macro_rules! lazy_bastard {
    ( 
        $(#[$meta: meta])*
        
        $vis:vis
        struct
        $name:ident
        $(<$life: lifetime>)?

        { $($field_vis:vis $field_name:ident : $field_type:ty $(=> $def: expr)? ),* $(,)? }
    ) => {
        
        $(
            #[$meta]
        )*
        
        $vis struct $name $(<$life>)? {
            $(
                $field_vis $field_name: $field_type,
            )*
        }
        
        impl $(<$life>)? Default for $name $(<$life>)? {
            fn default() -> Self {
                Self {
                    $(
                        $field_name: lazy_bastard!(@default $($def)?),
                    )*
                }
            }
        }
    };
    
    (@default $def:expr) => {
        $def
    };
    (@default) => {
        Default::default()
    };
}



/// an internal macro for other functions, returns input code without creating an inner scope
/// ```
/// use lazy_bastard::code_in_scope;
///
/// code_in_scope!(let mut v = 5);
/// v += 1;
///
/// // compiles to
/// let mut v = 5;
/// v += 1;
///
/// // instead of if using $x: block
/// { let mut v = 5; };
/// v += 1; // fails to compile
///
///
/// ```
#[macro_export]
macro_rules! code_in_scope {
    ($($code: tt)* ) => {
        $($code)*
    };
}


/// times a section of code printing the elapsed time, takes an optional name before the code
#[macro_export]
macro_rules! time_section_print {
    ($($name: literal, )? {$($code: tt)*}) => {
        let st_macro_inner_veriable = std::time::Instant::now();
        lazy_bastard::code_in_scope!($($code)*);
        println!("{} => {:?}", time_section_print!(@default $($name)? ), st_macro_inner_veriable.elapsed());
    };

    (@default $def:literal) => {
        $def
    };
    (@default) => {
        "Time duration"
    };
}