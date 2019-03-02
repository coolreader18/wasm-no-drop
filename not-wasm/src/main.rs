struct A;
impl Drop for A {
    fn drop(&mut self) {
        println!("dropping");
    }
}

fn main() {
    let _a = A;
    panic!("panic");
}
