

/// inline defaults for struct definitions because your lazy
#[macro_export]
macro_rules! lazy_bastard {
    ( 
        $(#[$meta: meta])*
        
        $vis:vis
        struct
        $name:ident
        $(<$life: lifetime>)?

        { $($field_name:ident : $field_type:ty $(=> $def: expr)? ),* $(,)? }
    ) => {
        
        $(
            #[$meta]
        )*
        
        $vis struct $name $(<$life>)? {
            $(
                $field_name: $field_type,
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
