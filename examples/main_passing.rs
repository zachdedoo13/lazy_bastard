#![deprecated]
#![allow(dead_code)]

use syn::{File, Item, parse_file};

fn main() {
   // let target = Path::new("examples/ttt.rs");
   // let file_str = fs::read_to_string(&target).unwrap();

   let file_str = "
      fn main() {
         in_one();
         in_two();

         let c = 3.1 + 9f32;
      }

      fn in_one() { in_two() }
      fn in_two() {}"
       .to_string();

   println!("{file_str}\n/////////////////////////////////////\n");

   let file_ast = parse_file(&file_str).unwrap();
   println!("File AST: {:#?}", file_ast);

   let main = find_function_with_name(&file_ast, "main").unwrap();
   println!("///////////////////\n{main:#?}");

   let internal = get_internal_functions(main);
}

fn find_function_with_name(file: &File, name: impl Into<String> + Clone) -> Option<&Item> {
   let main = file.items
       .iter()
       .find(|f| {
          if let Item::Fn(con) = *f {
             let raw_name = con.sig.ident.to_string();
             if raw_name == name.clone().into() { return true; }
          };
          false
       });
   main
}

fn get_internal_functions(func: &Item) {

}