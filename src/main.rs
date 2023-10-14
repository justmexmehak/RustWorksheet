mod worksheet1;
mod worksheet2;

// fn main() {
//     create();
// }

// fn create(){
//     // with type annotation
//     let v: Vec<i32> = Vec::new();

//     // declared with vec! macro
//     let v2 = vec![1, 2, 3];

//     // declared without type annotation
//     let mut v3 = Vec::new();

//     v3.push(5);
//     v3.push(6);
//     v3.push(7);

//     dbg!(&v);
//     dbg!(&v2);
//     dbg!(&v3);

//     let third: &i32 = &v2[2];
//     println!("{third}");
//     // type independance
//     let third = v2.get(2);
//     println!("{:?}", third);

//     let mut v4 = vec![1, 2, 3, 4, 5];
//     // let does_not_exist = &v4[100];
//     let does_not_exist = v4.get(100);
//     println!("{:?}", does_not_exist);

//     let first = &v4[0];
//     println!("{}", first);

//     v4.push(6);
//     dbg!(&v4);
    
//     for i in &v4{
//         println!("{}", i);
//     }

//     for i in &mut v4{
//         *i += 50;
//     }

//     // Dereference operator to access the value of i
//     for i in &v4{
//         println!("{}", i);
//     }
   

//    let mut index = 0;

//    while index < v4.len(){
//     v4[index] += 100;
//     index += 1;
//    }
//    println!("{:?}", v4);
// }

fn main(){
    let x: u32 = 1;
    let question: u32 = 18;    
   match x{
        1 => worksheet1::run(question),
        2 => worksheet2::run(question),
        _ => println!("Question set not implemented yet.")
   }
}