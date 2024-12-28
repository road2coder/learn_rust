// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// Lifetime annotations don’t change how long any of the references live. Rather, they describe the
// relationships of the lifetimes of multiple references to each other without affecting the lifetimes

fn main() {
    let s1 = "rust";
    let s2 = String::from("python");
    println!(
        "'{}' is the longer one between '{s1}' and '{s2}'",
        longer(s1, &s2)
    );

    let mut ie = ImportantExcerpt { part: "some str" };
    {
        let s = String::from("another part");
        ie.part = &s;
        println!("{:?}", ie);
    }
    // println!("{:?}", ie); // error: ie can not outlive s which ie.part borrowed from
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
