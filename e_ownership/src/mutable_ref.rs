let mut s1 = String::from("abc");
do_stuff(&mut s1);

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Hi", "");
}

// Mutable reference