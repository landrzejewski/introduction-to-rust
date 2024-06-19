// This function receives a closure parameter. 
// It only invokes the closure once, so the closure is bound to FnOnce.
fn receive_fnonce<F>(func: F) 
    where F: FnOnce() {

    func();
}

pub fn do_it() {

    println!("\nIn demo_closures_fnonce::do_it()");

    // Can pass in an FnOnce closure.
    let s1 = String::from("aaa");
    receive_fnonce(|| {
        println!("{}", s1);
        std::mem::drop(s1);
    });

    // Can pass in an FnMut closure.
    let mut s2 = String::from("bbb");
    receive_fnonce(|| {
        s2.push_str(" BBB");
        println!("{}", s2);
    });

    // Can pass in an Fn closure.
    let s3 = String::from("ccc");
    receive_fnonce(|| println!("{}", s3));
}