extern crate rand;
use rand::Rng;

fn selection_sort(a: &mut Vec<i32>) {
    for i in 0..a.len()-1 {
        let mut min = i;
        for j in i+1..a.len() {
            if a[j] < a[min] {
                min = j;
            }
        }
        let aux = a[i];
        a[i] = a[min];
        a[min] = aux;
    }
}

fn main() {
    let mut _a: Vec<i32> = (0..10).map(|_| { rand::thread_rng().gen_range(1..100)}).collect();
    
    print!("\nOriginal vector: ");
    for i in _a.iter() { print!("{} ", i); }
    selection_sort(&mut _a);
    print!("\nOrdered vector with Selection Sort: ");
    for i in _a.iter() { print!("{} ", i); }
}