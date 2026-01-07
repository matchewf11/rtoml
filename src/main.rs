use rtoml;

fn main() {
    let test = "me = \"hello\"\n#foo\ntoo = 1\n#foooooo\ngaa = true\nshee = false";
    match rtoml::parse(test) {
        Ok(vec) => {
            println!("{vec:?}");
            let rs = rtoml::token_list_to_map(&vec).unwrap();
            println!("{rs:?}");
        }
        Err(_) => println!("Error :("),
    }
}
