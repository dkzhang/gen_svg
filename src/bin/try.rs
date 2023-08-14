
fn main() {
    let mut vs = vec![1, 2, 3, 4, 5];

    println!("vs.len() {}", vs.len());
    vs.remove(1);
    println!("vs.len() {}", vs.len());
}