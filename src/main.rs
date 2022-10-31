fn main() {
    let days = [
        "First",
        "Second",
        "Third",
        "Fourth",
        "Fifth",
        "Sixth",
        "Seventh",
        "Eighth",
        "Ninth",
        "Tenth",
        "Eleventh",
        "Twelfth"
    ];
    let brought = [
        "A song and a Christmas tree",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes"
    ];

    let mut count = 1;

    for day in days {
        println!("On the {} day of Christmas", day);
        println!("My good friends brought to me");
        for i in (0..count).rev() {
            let gift = brought[i];
            println!("{}", gift);
        }
        count += 1;
        println!("\n");
    }
}
