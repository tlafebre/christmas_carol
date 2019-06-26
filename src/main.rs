fn main() {
    for iteration in 1..13 {
        let nth_time = nth(iteration).0;
        let mut presents_list = Vec::new();

        for i in 1..1 + iteration {
            presents_list.insert(0, nth(i).1)
        }
        
        let presents = presents_list.join("\n");

        println!("On the {} day of Christmas,\nMy true love sent to me\n{}\n", nth_time, presents);
    }
}

fn nth(n: u8) -> (String, String) {
    match n {
        1 => (String::from("First"), String::from("A partridge in a pear tree")),
        2 => (String::from("Second"), String::from("Two turtle-doves and")),
        3 => (String::from("Third"), String::from("Three French hens,")),
        4 => (String::from("Fourth"), String::from("Four calling birds,")),
        5 => (String::from("Fifth"), String::from("Five golden rings.")),
        6 => (String::from("Sixth"), String::from("Six geese a-laying,")),
        7 => (String::from("Seventh"), String::from("Seven swans a-swimming,")),
        8 => (String::from("Eighth"), String::from("Eight maids a-milking,")),
        9 => (String::from("Ninth"), String::from("Nine ladies dancing,")),
        10 => (String::from("Tenth"), String::from("Ten lords a-leaping,")),
        11 => (String::from("Eleventh"), String::from("Eleven pipers piping,")),
        12 => (String::from("Twelfth"), String::from("Twelve drummers drummings,")),
        _ => (String::from(""), String::from("")),
    }
}
