#[derive(Debug,Default)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug,Default)]
struct Rectangle2 {
    width: u32,
    height: u32,
}
fn main() {
    let reat1:Rectangle = Default::default();
    let reat2:Rectangle2 = Default::default();
    let reat3 = Rectangle2::default();
    // println!("{:?}",reat1);
    // println!("{:?}",reat2.height);
    // println!("{:?}",reat3);
    let s = String::from("这个是什么");
    let a = Some(s);
    match a {
        Some(ref s) => println!("{:?}",s),
        None => println!("None"),
    }
    // println!("{:?}",a)
}
//    ---------- fields in this struct fields `width` and `height` are never read