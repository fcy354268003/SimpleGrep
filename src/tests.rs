use super::*;

#[test]
fn one_result() {
    let query = String::from("fcy");
    let contents = String::from("\
        show me what you have?
        Fcy===
        tell me your target
        from fcy");
    if env::var("CASE_INSENSITIVE").is_err() {
        assert_eq!(vec![String::from("        Fcy==="), String::from("        from fcy")], search_insensitive(&query, &contents));
    } else {
        assert_eq!(vec![String::from("        from fcy")], search(&query, &contents));
    }
}

#[test]
fn show() {
    let action = |x| {
        println!("asdad");
        x + 1
    };
    let s = |x| x;
    action(2);
    s("sd");
}
