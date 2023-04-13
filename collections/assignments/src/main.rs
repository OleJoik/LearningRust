mod median;
mod mode;
mod pig_latin;

fn main() {
    /* Given a list of integers, use a vector and return the
    median (when sorted, the value in the middle position)
    and mode (the value that occurs most often; a hash map
    will be helpful here) of the list. */

    let integers = [4, 6, 4, 7, 8, 10, 11, 6, 2, 3, 4, 7, 7, 9];

    median::print(&integers);
    let mode = mode::mode(&integers);
    println!("Mode is {mode}");
    println!("Original integers are {:?}", integers);

    /* Convert strings to pig latin. The first consonant of
    each word is moved to the end of the word and “ay” is added,
    so “first” becomes “irst-fay.” Words that start with a
    vowel have “hay” added to the end instead (“apple”
    becomes “apple-hay”). Keep in mind the details about
    UTF-8 encoding! */

    let s1 = "adding some text in the first string literal";
    let s2 = "apples are not cats";

    pig_latin::print(s1);
    pig_latin::print(s2);
}
