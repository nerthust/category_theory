use std::collections::HashMap;

fn fib_naive(arg: u32) -> u32 {
    match arg {
        0 => 0,
        1 => 1,
        n => fib_naive(n - 1) + fib_naive(n - 2),
    }
}


struct Factorial {
    cache: HashMap<i32, i32>,
}

impl Factorial {
    fn new() -> Factorial {
        let cache: HashMap<i32, i32> =HashMap::new();
        Factorial { cache }
    }

    fn factorial(&mut self, n: i32) -> i32 {
        if self.cache.contains_key(&n) {
            return *self.cache.get(&n).unwrap();
        }

         match n {
             0 => 1,
             _ => {
                 let x = self.factorial(n - 1) * n;
                 self.cache.insert(n,x);

                 return x;
             }
         }
    }
}
