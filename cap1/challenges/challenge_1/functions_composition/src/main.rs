fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    //let multiply_and_add = compose(|x: f64| x * 2., |x: f64| x + 2.);
    //println!("{}", multiply_and_add(2.));

    let identity_function = |x: f64| x;
    let fx = |x: f64| x * 2.;

    //(f ◦ idA)(a) = f (idA(a)) = f (a)
    let composition_0 = compose(fx, identity_function);
    //(idB ◦ f )(a) = idB (f (a))) = f (a)
    let composition_1 = compose(identity_function, fx);

    println!("{}", composition_0(2.));
    println!("{}", composition_1(2.));
}
