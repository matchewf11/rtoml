use rtoml::{Value, parse};

fn main() {
    let test = r#"
        me = "hello"
        # foo
        too = 1 # foo
        neg_two = -2 # foo
        #foooooo
        gaa = true
        shee = false
        float_test0 = 11.
        float_test1 = 1.1
        float_test2 = .11
        "#;

    let map = parse(test).expect("should pass");

    match map.get("too").expect("should pass") {
        &Value::Int(x) => assert!(x == 1),
        _ => panic!("failed"),
    }

    println!("{map:?}");
}
