use env_logger::Builder;
use log::LevelFilter;
use std::time::SystemTime;

use log::info;
mod bn64;
use bn64::Bn64;

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();
    /*
    let mersenne: usize = 3217;
    let mut p = bn64::mersenne(mersenne);
    p.to_hex();
    let mut p_1 = p.clone();
    p_1.sub_at(0, 1);
    // p_1.add_at(0, 2);
    let mut bn0: Bn64 = Bn64::from(String::from("ace"));

    let start = SystemTime::now();
    let r = bn64::npmod3(&mut bn0, &mut p_1, &mut p);
    let end = SystemTime::now();
    info!(
        "Time elapsed {:?} milliseconds",
        end.duration_since(start).unwrap().as_millis()
    );
    */
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
    let length = x.len();
    quick_sort2(&mut x, 0, length);
    info!("{:?}", x);

    /*
       let mut a = vec![1, 2, 3];
       let mut b: Vec<i32> = vec![0; 3];
       b.copy_from_slice(&a);
       let (left, right) = b.split_at_mut(1);
       info!("{:?}", left);


    let mut n: Vec<i32> = vec![1, 5, 10, 50, 100];
    let v = 200;
    let x = notes(&mut n, v);
    info!("{:?}", x);

     */
}

// index >= range_from, index < range_to
fn quick_sort1<T: Ord + Clone>(v: &mut [T], range_from: usize, range_to: usize) {
    if range_from + 1 >= range_to {
        return;
    }
    if range_from + 2 == range_to {
        if v[range_from] <= v[range_to - 1] {
            return;
        } else {
            v.swap(range_from, range_to - 1);
            return;
        }
    }

    let pivot = v[range_to - 1].clone();
    let mut from: usize = range_from;
    let mut to: usize = range_to - 2;
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
    v.swap(from, range_to - 1);
    quick_sort1(v, range_from, from);
    quick_sort1(v, from + 1, range_to);
}

fn quick_sort2<T: Ord + Clone>(v: &mut [T], range_from: usize, range_to: usize) -> &mut [T]{
    if range_from + 1 >= range_to {
        return v; //why can't be 'v'
    }
    if range_from + 2 == range_to {
        if v[range_from] <= v[range_to - 1] {
            return v;
        } else {
            v.swap(range_from, range_to - 1);
            return v;
        }
    }

    let pivot = v[range_to - 1].clone();
    let mut from: usize = range_from;
    let mut to: usize = range_to - 2;
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
    v.swap(from, range_to - 1);
    quick_sort2(v, range_from, from);
    quick_sort2(v, from + 1, range_to);
    return v;
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

/*
fn notes(n: &mut [i32], v: i32) -> u32 {
    // info!("n={:?}, v={:?}", n, v);
    if v == 0 {
        return 1;
    }
    if v < 0 {
        return 0;
    }
    //if there is only 1 element
    if n.len() == 1 {
        if v % n[0] == 0 {
            return 1;
        } else {
            return 0;
        }
    }
    n.sort_by(|a, b| b.cmp(a));
    let mut n_copy: Vec<i32> = vec![0; n.len()];
    n_copy.copy_from_slice(n);
    let (left, right) = n_copy.split_at_mut(1);
    let max = left[0];
    return notes(n, v - max) + notes(right, v);
}
*/
