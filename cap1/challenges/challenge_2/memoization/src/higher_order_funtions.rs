// dynamic dispatch using trait objects
fn apply_pure<F: Fn(T1) -> T2, T1, T2> (f: & F, x: T1) -> T2 {
    f(x)
}

// dynamic dispatch using trait objects
fn apply_mut<F: FnMut(T1) -> T2, T1, T2> (f: &mut F, x: T1) -> T2 {
    f(x)
}

// static dispatch, FnOnce is not object safe and cannot be made into a trait object
fn apply_once<F: FnOnce(T1) -> T2, T1, T2> (f: F, x: T1) -> T2 {
    f(x)
}

// factory returns a heap allocated moved closure that is a trait object
fn factory (n: f64) -> Box<FnMut(f64) -> f64> {
    let mut acc = n;
    Box::new(move |i: f64| {
        acc += i; 
        acc
    })
}

fn display_solutions() {
    let closure_immutable_pure = |x: i32| x + 1;
    
    let result1 = apply_pure(& closure_immutable_pure, 1);
    // let result2 = apply_mut(&mut closure_immutable_pure, 2); // not typeable
    let result3 = apply_once(closure_immutable_pure, 3);
    
    let mut closure_mutable_pure = |x: i32| x + 1;
    
    let result4 = apply_pure(& closure_mutable_pure, 4);
    let result5 = apply_mut(&mut closure_mutable_pure, 5);
    let result6 = apply_once(closure_mutable_pure, 6);
    
    let mut state1 = 1;
    let closure_immutable_effectful = |x: i32| {
        state1 += 1;
        x + 1
    };
    
    // let result7 = apply_pure(& closure_immutable_effectful, 7); // not typeable
    // let result8 = apply_mut(&mut closure_immutable_effectful, 8); // not typeable
    let result9 = apply_once(closure_immutable_effectful, 9);
    
    let mut state2 = 2;
    let mut closure_mutable_effectful = |x: i32| {
        state2 += 1;
        x + 1
    };
    
    // let result10 = apply_pure(& closure_mutable_effectful, 10); // not typeable
    let result11 = apply_mut(&mut closure_mutable_effectful, 11);
    let result12 = apply_once(closure_mutable_effectful, 12);
    
    let state3 = 3;
    let closure_immutable_moved_pure = move |x: i32| {
        x + state3
    };
    
    let result13 = apply_pure(& closure_immutable_moved_pure, 13);
    // let result14 = apply_mut(&mut closure_immutable_moved_pure, 14); // not typeable
    let result15 = apply_once(closure_immutable_moved_pure, 15);
    
    let mut state4 = 4;
    let closure_immutable_moved_effectful = move |x: i32| {
        state4 += 1;
        x + state4
    };
    
    // let result16 = apply_pure(& closure_immutable_moved_effectful, 16); // not typeable
    // let result17 = apply_mut(&mut closure_immutable_moved_effectful, 17); // not typeable
    let result18 = apply_once(closure_immutable_moved_effectful, 18);
    
    let mut state5 = 5;
    let mut closure_mutable_moved_effectful = move |x: i32| {
        state5 += 1;
        x + state5
    };
    
    // let result19 = apply_pure(& closure_mutable_moved_effectful, 19); // not typeable
    let result20 = apply_mut(&mut closure_mutable_moved_effectful, 20);
    let result21 = apply_once(closure_mutable_moved_effectful, 21);
    
    let mut closure_mutable_moved_effectful_boxed = factory(10.0);

    // let result22 = apply_pure(&&* closure_mutable_moved_effectful_boxed, 22.0); // not typeable
    let result23 = apply_mut (&mut&mut* closure_mutable_moved_effectful_boxed, 23.0);
    let result24 = apply_once(&mut* closure_mutable_moved_effectful_boxed, 24.0);
    
    fn function_immutable_pure (x: i32) -> i32 {
        x + 1
    }
    
    let result25 = apply_pure(& function_immutable_pure, 25);
    let result26 = apply_mut(&mut function_immutable_pure, 26); // look I can coerce it to be mutable
    let result27 = apply_once(function_immutable_pure, 27);
    let result28 = apply_once(function_immutable_pure, 28); // look I can run it twice!
    
    fn function_immutable_pure_reference (x: &i32) -> i32 {
        x + 1
    }
    
    let result29 = apply_pure(& function_immutable_pure_reference, &29);
    let result30 = apply_mut(&mut function_immutable_pure_reference, &30);
    let result31 = apply_once(function_immutable_pure_reference, &31);
    let result32 = apply_once(function_immutable_pure_reference, &32);
    
    let mut state6 = 6;
    fn function_immutable_effectful_reference (x: &mut i32) -> () {
        *x += 1;
    }
    
    let result33 = apply_pure(& function_immutable_effectful_reference, &mut state6);
    let result34 = apply_mut(&mut function_immutable_effectful_reference, &mut state6);
    let result35 = apply_once(function_immutable_effectful_reference, &mut state6);
    let result36 = apply_once(function_immutable_effectful_reference, &mut state6);
    
    let closure_immutable_effectful_reference = |x: &mut i32| -> () { (* x) += 1 };
    
    let result37 = apply_pure(& closure_immutable_effectful_reference, &mut 37);
    // let result38 = apply_mut(&mut closure_immutable_effectful_reference, &mut 38); // not typeable
    let result39 = apply_once(closure_immutable_effectful_reference, &mut 39);

    let mut state7 = 7;
    let closure_immutable_effectful_reference2 = |x: &mut i32| -> () { (* x) += 1 };
    
    let result40 = apply_pure(& closure_immutable_effectful_reference2, &mut state7);
    // let result41 = apply_mut(&mut closure_immutable_effectful_reference2, &mut state7); // not typeable
    let result42 = apply_once(closure_immutable_effectful_reference2, &mut state7);
    
    let result43 = apply_pure(& |x: i32| x + 1, 43);
    let result44 = apply_mut(&mut |x: i32| x + 1, 44);
    let result45 = apply_once(|x: i32| x + 1, 45);
    
    println!("1: {}", result1);
    // println!("2: {}", result2);
    println!("3: {}", result3);
    println!("4: {}", result4);
    println!("5: {}", result5);
    println!("6: {}", result6);
    // println!("7: {}", result7);
    // println!("8: {}", result8);
    println!("9: {}", result9);
    // println!("10: {}", result10);
    println!("11: {}", result11);
    println!("12: {}", result12);
    println!("13: {}", result13);
    // println!("14: {}", result14);
    println!("15: {}", result15);
    // println!("16: {}", result16);
    // println!("17: {}", result17);
    println!("18: {}", result18);
    // println!("19: {}", result19);
    println!("20: {}", result20);
    println!("21: {}", result21);
    // println!("22: {}", result22);
    println!("23: {:.*}", 1, result23);
    println!("24: {:.*}", 1, result24);
    println!("25: {}", result25);
    println!("26: {}", result26);
    println!("27: {}", result27);
    println!("28: {}", result28);
    println!("29: {}", result29);
    println!("30: {}", result30);
    println!("31: {}", result31);
    println!("32: {}", result32);
    println!("33: {:?}", result33);
    println!("34: {:?}", result34);
    println!("35: {:?}", result35);
    println!("36: {:?}", result36);
    println!("37: {:?}", result37);
    // println!("38: {:?}", result38);
    println!("39: {:?}", result39);
    println!("40: {:?}", result40);
    // println!("41: {:?}", result41);
    println!("42: {:?}", result42);
    println!("43: {}", result43);
    println!("44: {}", result44);
    println!("45: {}", result45);
}
