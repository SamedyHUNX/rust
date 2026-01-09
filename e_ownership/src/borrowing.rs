let s1 = String::from("abc");
do_stuff(&s1);
println!("{}", s1);

fn do_stuff(s: &String) {
    // do stuff
}

// Borrowing: only the ref is passed and not the value itself
// Ref cannot be pointed to null (must always be a valid value).