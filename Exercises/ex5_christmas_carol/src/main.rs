fn main() {
    println!("---- The Twelve Days of Christmas ----\n");

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelve",
    ];
    let animals = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    // Generates the song.
    for (index, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas my true love sent to me");
        if index != 0 {
            for number in 1..=index {
                println!("{},", animals[number]);
            }
            println!("And {}.\n", animals[0].to_lowercase());
        } else {
            println!("{}.\n", animals[0]);
        }
    }
}
