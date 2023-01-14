// we can define a module inside this module
pub mod level1 {
  // if we wanna expose functions, structus or whatever to external modules, that must 
  // be done explicitly by using the 'pub' keyword
  pub fn level1_print() {
    println!("level1");
  }

  pub mod level2 {
    pub fn level2_print() {
      println!("level 2");
    } 
  }

  // this module is not marked as pub, so it is private by default
  mod private_module {
    // private stuff
  }
}
