mod screen;
mod triangle;
mod transformation;
use rand::Rng;
use triangle::{Triangle, Point};
use screen::{Screen};

fn main() {
    let mut rng = rand::rng();
    let t: Triangle = rng.random(); 
    println!("{:#?}", t); // 变量名必须匹配
}
