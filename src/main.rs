fn main() {
    for iteration in 1..13 {

        println!("On the {} day of Christmas,\nMy true love sent to me", day(iteration));

        let presents = (1..=iteration).rev().map(present);
        for present in presents {
            println!("{}", present);
        }
        println!(""); 
    }
}

fn day(n: u8) -> &'static str {
    match n {
        1 => "First",
        2 => "Second",
        3 => "Third",
        4 => "Fourth",
        5 => "Fifth",
        6 => "Sixth",
        7 => "Seventh",
        8 => "Eighth",
        9 => "Ninth",
        10 => "Tenth",
        11 => "Eleventh",
        12 => "Twelfth",
        _ => "",
    }
}

fn present(n: u8) -> &'static str {
    match n {
        1 => "A partridge in a pear tree",
        2 => "Two turtle-doves and",
        3 => "Three French hens,", 
        4 => "Four calling birds,",
        5 => "Five golden rings.",
        6 => "Six geese a-laying,",
        7 => "Seven swans a-swimming,",
        8 => "Eight maids a-milking,",
        9 => "Nine ladies dancing,",
        10 => "Ten lords a-leaping,",
        11 => "Eleven pipers piping,",
        12 => "Twelve drummers drummings,",
        _ => "",
    }
}
