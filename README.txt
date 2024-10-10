inline defaults for struct definitions if you’re lazy, plus some other stuff, probably.

makes creating simple structs that don’t need a constructor like settings structs much clearer

somewhat second guessing the name, but it’s too late to change it

```
    use lazy_bastard::lazy_bastard;

    lazy_bastard!(
       #[derive(Clone, Debug)]
       pub struct MyStruct<'a> { // lifetime is just for the example
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


    impl MyStruct {
        fn new() -> Self { Self::default() }
    }


    // Compiles into //

    #[derive(Clone, Debug)]
    pub struct CompiledStruct<'a> {
       normal: i32,
       function_call: String,
       automatic: f64,
       scoped: f32,
    }
    impl<'a> Default for CompiledStruct<'a> {
       fn default() -> Self {
          Self {
             normal: 100_324,
             function_call: ("test".into()),
             automatic: Default::default(),
             scoped: {
                let c: f32 = 1.2;
                let v = c.abs().sin().sin().sqrt();
                0.1 * v
             },
          }
       }
    }

```