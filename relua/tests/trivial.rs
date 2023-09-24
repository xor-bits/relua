use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use relua::{State, Value};

//

#[test]
pub fn increment_relua_to_rust() {
    let mut lua = State::new();

    let called = Rc::new(AtomicBool::new(false));
    lua.set("inc", {
        let called = called.clone();
        move |_| called.store(true, Ordering::SeqCst)
    });

    lua.run("inc()").unwrap();

    assert!(called.load(Ordering::SeqCst));
}

#[test]
pub fn n_params_relua_to_rust() {
    let mut lua = State::new();

    let n = AtomicUsize::new(0);
    lua.set("print", move |args: Vec<Value>| {
        assert_eq!(n.fetch_add(1, Ordering::SeqCst), args.len());
    });

    lua.run(r#"print() print "test" print(0.4, .4) print(4., false, nil)"#)
        .unwrap();
}

#[test]
pub fn test_swap() {
    let mut lua = State::new();

    lua.set("test_swap", move |args: Vec<Value>| {
        let arg0 = args[0].as_number().unwrap();
        let arg1 = args[1].as_number().unwrap();
        assert!((arg0 - 2.0).abs() <= f64::EPSILON);
        assert!((arg1 - 1.0).abs() <= f64::EPSILON);
    });

    lua.run("x, y = 1, 2 x, y = y, x test_swap(x, y)").unwrap();
}
