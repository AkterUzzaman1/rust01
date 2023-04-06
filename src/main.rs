use std::result;

//video6
fn ifelse(){
  let a=120;

  if a>99{
    println!("big number");
    if a>200{
      println!("huge number");
    }else{
      println!("not huge number");
    }

  }
  else {
      println!("small number");
  }
}
//video8
// fn lop(){
//   let mut a=0;
//   loop{
//     if a==5{
//       break;
//     }
//     println!("{:?}",a);
//     a= a + 1;
//   }
// }
//video 12
fn add(a:i32, b:i32)->i32{
  a+b
}
fn display(result:i32){
  println!("total={}",result);
}
//video14
fn bools(){
  let my_bool=false;
  if my_bool==true{
    println!("Hello");
  }else {
      println!("Good bye!!");
  }
}

fn main() {
  bools();
  //ifelse();
//fn lop();
// let result=add(3,4);
// display(result);

 }
