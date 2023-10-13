fn main(){
    let x=123;
    pt(&x);
}
fn pt(x:&i32){
    println!("{}",*x);
}