fn main() {
    let file = include_str!("../input1.txt");
    let mut counter = 0;
    file.lines().for_each(|line| {
        counter = counter + 1;
    });
    print!("{}", counter)
}
