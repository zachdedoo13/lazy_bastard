use lazy_static::lazy_static;
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

        $($suffix: tt)*
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

        lazy_bastard::code_in_scope!($($suffix)*);
    };


    // dupe for = exactly the same but the 'only' way to accept both => and =
    (
        $(#[$meta: meta])*

        $vis:vis
        struct
        $name:ident
        $(<$life: lifetime>)?

        { $($field_vis:vis $field_name:ident : $field_type:ty $(= $def: expr)? ),* $(,)? }

        $($suffix: tt)*
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

        lazy_bastard::code_in_scope!($($suffix)*);
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


/// uses print or store, if store holds the time in a
/// variable call what was input as the name
/// ```
/// use lazy_bastard::timer;
///
/// timer!(b, STORE, {
///       let mut v = 0;
///       for i in 0..1_000_000_i128 {
///          v += i.pow(4);
///       }
///    });
///
///     // manually use timings
///    println!("{b}");
///
/// // or just print
/// // value is dropped after print
/// timer!(test, PRINT, {
///       let mut v = 0;
///       for i in 0..1_000_000_i128 {
///          v += i.pow(4);
///       }
///    });
/// ```
#[macro_export]
macro_rules! timer {
    ($name: ident, PRINT, {$($code: tt)*}) => {
        let temp_time_veriable_that_knowone_should_use_as_a_veriable_name = lazy_bastard::timers::start_timer( stringify!($name) );
        lazy_bastard::code_in_scope!($($code)*);
        println!("{}", lazy_bastard::timers::end_timer(temp_time_veriable_that_knowone_should_use_as_a_veriable_name));
        drop(temp_time_veriable_that_knowone_should_use_as_a_veriable_name);
    };

    ($name: ident, STORE, {$($code: tt)*}) => {
        let temp_time_veriable_that_knowone_should_use_as_a_veriable_name = lazy_bastard::timers::start_timer( stringify!($name) );
        lazy_bastard::code_in_scope!($($code)*);
        let $name = lazy_bastard::timers::end_timer(temp_time_veriable_that_knowone_should_use_as_a_veriable_name);
        drop(temp_time_veriable_that_knowone_should_use_as_a_veriable_name);
    };
}


/// ``Arc<RwLock<T>>``
/// ```
/// use lazy_bastard::{init_global_mut, safe_get_mut};
///
/// init_global_mut!(DATA: f32 => 0.0);
///
/// safe_get_mut(&DATA, |mut d| {
///      *d += 1.0;
/// });
/// ```
#[macro_export]
macro_rules! init_global_mut {
    ($name: ident: $ty: ty => $init: expr) => {
        lazy_static::lazy_static!(
            static ref DATA: std::sync::Arc<std::sync::RwLock<$ty>> = std::sync::Arc::new(std::sync::RwLock::new($init));
        );
    };
}
