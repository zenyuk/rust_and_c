extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    let x = unsafe { add(64, 35) };
    println!("{}", x);
}
 
 /*
  * compile with:
  * rustc -l foo -L . main.rs
  *
  * run with:
  * env LD_LIBRARY_PATH="$LD_LIBRARY_PATH:." ./main
  *
  * */
