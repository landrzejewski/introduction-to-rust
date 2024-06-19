// This function receives a closure parameter. 
// It invokes the closure many times and doesn't allow mutable capture, so the closure is bound to Fn.
fn receive_fn<F>(func: F) 
    where F: Fn() {

    func();
    func();
}

pub fn do_it() {

    println!("\nIn demo_closures_fn::do_it()");

    // Can't pass in an FnOnce closure.
    // let s1 = String::from("aaa");
    // receive_fn(|| {
    //     println!("{}", s1);
    //     std::mem::drop(s1);
    // });

    // Can't pass in an FnMut closure.
    // let mut s2 = String::from("bbb");
    // receive_fn(|| {
    //     s2.push_str(" BBB");
    //     println!("{}", s2);
    // });

    // Can pass in an Fn closure.
    let s3 = String::from("ccc");
    receive_fn(|| println!("{}", s3));
}