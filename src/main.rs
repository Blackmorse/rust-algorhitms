extern crate rand;

use rand::Rng;
use std::time::{Duration, SystemTime};

trait SortAlgorhitm {
    fn sort(vec: &mut Vec<impl Ord>);

    fn less<T>(a: T, b: T) -> bool where T: Ord {
        a < b
    }

    fn exch<T>(vec: &mut Vec<T>, a: usize, b: usize) where T: Ord {
        vec.swap(a, b);
    }
}

struct Bubble {}
struct Selection {}
struct Insertion {}
struct ShellSort {}
struct InsertionWithoutExchangesSort {}
struct InsertionWithSentinel {}

impl SortAlgorhitm for Bubble  {
    fn sort(vec: &mut Vec<impl Ord>) {
        let n = vec.len();
        for _ in 0..n {
            for i in 1..n {
                if vec[i] < vec[i - 1] {
                    vec.swap(i, i - 1);
                }
            }
        }
    }
}

impl SortAlgorhitm for Selection {
    fn sort(vec: &mut Vec<impl Ord>) {
        let n = vec.len();

        for i in 0..n {
            let mut min = vec.get(i).unwrap();
            let mut min_index = i;
            for j in i+1..n {
                let c = vec.get(j).unwrap();
                if c < min {
                    min_index = j;
                    min = c;
                }
            }
            vec.swap(min_index, i);
        }
    
    }
}

impl SortAlgorhitm for Insertion {
    fn sort(vec: &mut Vec<impl Ord>) {
        let n = vec.len();
        
        for i in 1..n {
            let mut j = i;
            while j > 0 && vec[j] < vec[j - 1] {
                vec.swap(j, j-1);
                j = j - 1;
            }
        }
    }
}

impl SortAlgorhitm for ShellSort {
    fn sort(vec: &mut Vec<impl Ord>) {
        let n = vec.len();
        let mut h = 1;
        while h < n / 3 {
            h = 3 * h + 1;
        }

        while h >= 1 {
            for i in h..n {
                let mut j = i;              
                while (j as i32) - (h as i32) >= 0 && vec[j] < vec[j - h] {
                    vec.swap(j, j - h);
                    j -= h;
                }
            }    
            h /= 3;
        }
    }
}

impl SortAlgorhitm for InsertionWithoutExchangesSort {
    fn sort(vec: &mut Vec<impl Ord>) {
        let n = vec.len();

        for i in 1..n {
            let el = vec.get(i).unwrap();
            let mut j = i;
            while j > 0 && el < &vec[j - 1] {
                j -= 1;
            }

             vec[j..=i].rotate_right(1);
            //  vec.insert(j, el);
        }
    }
}

impl SortAlgorhitm for InsertionWithSentinel {
    fn sort(vec: &mut Vec<impl Ord>) {
        let n = vec.len();

        let mut min_index = 0;
        let mut min_value = &vec[0];

        for i in 1..n {
            if &vec[i] < min_value {
                min_value = &vec[i];
                min_index = i;
            }
        }
        vec.swap(0, min_index);

        for i in 1..n {
            let mut j = i;
            while vec[j] < vec[j - 1] {
                vec.swap(j, j-1);
                j = j - 1;
            }
        }
    }
}

fn generate_vec(n: usize) -> Vec<i32> {
    let mut vec = Vec::new();

    let mut rng = rand::thread_rng();

    for i in 0..n {
        let number: i32 = rng.gen();
        vec.push(number);

        vec.push(n as i32 - i as i32)

        // vec.push(10);
    }

    vec
}

fn check_sorted(vec: Vec<impl Ord>) -> bool {
    for i in 1..vec.len() {
        if vec[i] < vec[i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let n: usize = 3000;
    let t = 20;

    let mut t1: u128 = 0;
    let mut t2: u128 = 0;

    for _ in 0..t {
        let mut vec = generate_vec(n);
        let time = SystemTime::now();
        InsertionWithSentinel::sort(&mut vec);
        let elaps = time.elapsed().unwrap().as_millis();
        check_sorted(vec);
        t1 += elaps;

        let mut vec = generate_vec(n);
        let time = SystemTime::now();
        Insertion::sort(&mut vec);
        let elaps = time.elapsed().unwrap().as_millis();
        check_sorted(vec);
        t2 += elaps;
    }

    let res = t1 as f64 / t2 as f64;

    println!{"1 / 2 = {}", res}
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_vec() -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(5);
        v.push(3);
        v.push(0);
        v.push(-2);
        v.push(4);

        v
    }

    fn check(vec: Vec<i32>) {
        assert_eq!(vec[0], -2);
        assert_eq!(vec[1], 0);
        assert_eq!(vec[2], 1);
        assert_eq!(vec[3], 2);
        assert_eq!(vec[4], 3);
        assert_eq!(vec[5], 4);
        assert_eq!(vec[6], 5);
    }

    #[test]
    fn test_bubble() {
        let mut vec = get_vec();

        Bubble::sort(&mut vec);

        check(vec);
    }

    #[test]
    fn test_insertion() {
        let mut vec = get_vec();

        Insertion::sort(&mut vec);

        check(vec);
    }

    #[test]
    fn test_shell() {
        let mut vec = get_vec();

        ShellSort::sort(&mut vec);

        check(vec);
    }

    #[test]
    fn test_insertion_without_exchanges() {
        let mut vec = get_vec();

        InsertionWithoutExchangesSort::sort(&mut vec);

        check(vec);
    }

    #[test]
    fn test_insertion_with_sentinel() {
        let mut vec = get_vec();

        InsertionWithSentinel::sort(&mut vec);

        check(vec);
    }
}