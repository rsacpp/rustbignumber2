use env_logger::Builder;
use log::LevelFilter;
use std::time::SystemTime;

use log::info;
mod bn64;
use bn64::Bn64;

// #[tokio::main]
fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mersenne: usize = 3217;
    let mut p = bn64::mersenne(mersenne);

    p.to_hex();
    let mut p_1 = p.clone();
    // p_1.sub_at(0, 1);
    let mut bn0: Bn64 = Bn64::from(String::from("ace"));

    let start = SystemTime::now();
    let r = bn64::npmod2(&mut bn0, &mut p_1, &mut p);
    let end = SystemTime::now();
    info!("Time elapsed {:?}", end.duration_since(start));
    r.to_hex();

    /*
    let start = SystemTime::now();
    let r0 = bn64::npmod2(&mut bn0, &mut p_1, &mut p);
    let end = SystemTime::now();
    info!("Time elapsed {:?}", end.duration_since(start));
    r0.to_hex();

       let mut p_2 = bn128::mersenne(mersenne);
       p_2.to_hex();
       let mut p_3 = p_2.clone();
       p_3.sub_at(0, 1);
       let mut b128: Bn128 = Bn128::from(String::from("abcdef0123456789"));
       let start = SystemTime::now();
       let r = bn128::npmod2(&mut b128, &mut p_3, &mut p_2);
       let end = SystemTime::now();
       info!("Time elapsed {:?}", end.duration_since(start));
       r.to_hex();

    */
    let mut x = vec![
        1, 3, 4, 5, 7, 9, 8, 7, 3, 2, 1, 10, 100, 0, -3, 8, 7, 7, 7, 7, 7, 7, 7,
    ];
    quick_sort(&mut x);
    info!("{:?}", x);
}

fn quick_sort<T: Ord + Clone>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    if v.len() == 2 {
        if v[0] <= v[1] {
            return;
        } else {
            v.swap(0, 1);
            return;
        }
    }
    let pivot = v[v.len() - 1].clone();
    let mut from: usize = 0;
    let mut to: usize = v.len() - 2;
    while from < to {
        while v[from] <= pivot && from <= to {
            from += 1;
        }
        while v[to] > pivot && to > 0 {
            to -= 1;
        }
        if from < to {
            v.swap(from, to);
        }
    }
    v.swap(from, v.len() - 1);

    let (left, right) = v.split_at_mut(from);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}
