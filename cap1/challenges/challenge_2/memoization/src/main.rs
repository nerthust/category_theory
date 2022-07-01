use std::collections::HashMap;
use rand_distr::{Normal, Distribution};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

struct Memoize {
    cache: HashMap<i32, i32>,
}

impl Memoize {
    fn new() -> Memoize {
        let cache: HashMap<i32,i32> = HashMap::new();
        Memoize { cache }
    }

    fn apply_pure<F: Fn(i32) -> i32> (&mut self, f: & F, x: i32) -> i32 {
        if self.cache.contains_key(&x) {
            return *self.cache.get(&x).unwrap();
        }else {
            self.cache.insert(x, f(x));
            f(x)
        }
    }

    fn random_number(&mut self, x: i32) -> i32 {
        if self.cache.contains_key(&x) {
            return *self.cache.get(&x).unwrap();
        }else {
            let mut r = StdRng::seed_from_u64(x.try_into().unwrap()); 
            let n2: i32 = r.gen();
            self.cache.insert(x, n2);
            return n2;
        }
    }
}

fn main() {
    let mut memoize = Memoize::new();
    let random_number_0 = memoize.random_number(2);
    let random_number_1 = memoize.random_number(100);
    let random_number_2 = memoize.random_number(2);
    println!("{:?} - {:?} - {:?}", random_number_0, random_number_1, random_number_2);
 
    //let mut memoize = Memoize::new();
    //let closure_immutable_pure = |x: i32| x + 1;
    //let result = memoize.apply_pure(& closure_immutable_pure, 1);
    //println!("{}", result);
}
