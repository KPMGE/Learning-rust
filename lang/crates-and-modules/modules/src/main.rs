// in rust, we can create modules, that's the way we get organized.
// we can create a module an put all the code there or, we can define a module
// and put the source code in its dedicated file, which is usually better.
// when we define a module, using the mod keyword, rust looks for a file with 
// the name of the module we've defined or a folder with a mod.rs file, which is
// like index.js for javascript.
//
// when using modules, we can either spell out the whole module and its submodules to 
// get to a function, or we can use the 'use' keyword which brings in a certain module,
// trait, function, struct... to the current module.
//
// when using 'use', we're using absolute paths, if we wanna refer to the super module, 
// we can use the 'super' keyword. Refering to the current module is made by using the 'self'
// keyword
//
//
// NOTE: It's a good practice not to import functions directly, but intead, import modules.
//
// NOTE: We can define an alias using the 'type' keyword
//
// NOTE: Sometimes we wanna include everything from a module, in rust there is a pattern that says 
// we should export a 'prelude' module that exports the other modules. So that, we can import
// everything by using module::prelude::*;
//
// NOTE: When we use the a const variable, it's the same as #define in C. When using const static, 
// it makes a static variable, that lives as long as the program does. The first a approach is
// suitable for magic numbers and stuff like that, the second one is better when we got a huge
// amout of data or we needa borrow a reference to that variable.
// 
// we define a module 'single_file', rust looks for a 'single_file.rs' or 'single_file/mod.rs',
// if none of them or both of them exit, rust throws an error.
mod single_file;
// define that we've got a directory module, again rust looks for a 'directory.rs' or 'directory/mod.rs file'
mod directory;

// we can use the 'use' keyword which create an alias for single_file::level1
use single_file::level1;
use single_file::level1::level2;
use directory::dlevel1;
use directory::dlevel2;

fn main() {
  // we can spill out the whole thing
  single_file::level1::level1_print();
  // or if using the 'use' keyword
  level1::level1_print();
  // it's the same for submodules
  level1::level2::level2_print();
  level2::level2_print();

  // the idea is the same when we use directories
  dlevel1::dlevel1_print();
  dlevel2::dlevel2_print();
}
