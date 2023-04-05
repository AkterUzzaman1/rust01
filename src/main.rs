fn main() {
  //  println!("Hello, world!");
//video 1-3
// let two=2;//int
// let hello= "hello ";//string
// let j='j';//car
// let my_half=4.5;//float/doublke
// let mut mane="joy";//mutable
// let qait=false;//boolain
// let k=j;//j='j';

let x= add(1,1);
println!("x {}",x);
let y=add(2,3);
println!("y {}",y);
let z=add(x,y);
println!("z {}",z);
macros();
}
//vodeo-4
fn add(a:i32 ,b:i32) ->i32 {
  a+b
}
//video 5
fn macros(){
    let life=42;
    println!("hello");
    println!("life={}",life);
    println!("{:?} {:?}",life,life);
}