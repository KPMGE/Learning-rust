pub fn dlevel2_print() {
  println!("directory level 2 submodule!");
}
// we wont use this method and we want rust to shut up about it, 
// we can do that by anotating this method with the allow attribute
#[allow(dead_code)]
fn private_dlevel2() {
  // private stuff
}
