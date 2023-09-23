use relua::{State, Value};

//

#[test]
pub fn trivial() {
    let mut lua = State::new();

    lua.set("inc", 23);
    assert_eq!(lua.get("inc"), Value::Number(23.0));
}
