use rtoml::{Value, parse};

fn main() {
    let test = r#"
        me = "hello"
        # foo
        too = 1 # foo
        #foooooo
        gaa = true
        shee = false
        "#;

    let map = parse(test).expect("should pass");

    match map.get("too").expect("should pass") {
        &Value::Int(x) => assert!(x == 1),
        _ => panic!("failed"),
    }

    println!("{map:?}");
}
