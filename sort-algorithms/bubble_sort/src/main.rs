extern crate rand;
use rand::Rng;

fn bubble_sort(a: &mut Vec<i32>) {
    for i in 0..a.len()-1 {
        for j in 0..a.len()-1-i {
            if a[j+1] < a[j] { 
                let aux = a[j];
                a[j] = a[j+1];
                a[j+1] = aux;
            }
        }
    }
}

fn main() {
    let mut _a: Vec<i32> = (0..10).map(|_| { rand::thread_rng().gen_range(1..100)}).collect();
    
    print!("\nOriginal vector: ");
    for i in _a.iter() { print!("{} ", i); }
    bubble_sort(&mut _a);
    print!("\nOrdered vector with Bubble Sort: ");
    for i in _a.iter() { print!("{} ", i); }
}