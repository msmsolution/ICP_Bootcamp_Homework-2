fn main() {
    //Create a new instance of the wordCounter struct

    let text = "Hey, I'm learning Rust Programming Language at Rust Bootcamp!";
    let word_counter = wordCounter::new(text);

    //Print out the number of words in the text
    println!("The number of words in the text is: {}", word_counter.count_words());
}

//Implement the wordCounter struct
struct wordCounter {
    text: String,
}

//Implement the new function and the count_words function
impl wordCounter {
    fn new(text: &str) -> wordCounter {
        wordCounter {
            text: text.to_string(),
        }
    }
    fn count_words(&self) -> usize {
        let num_words = self.text.split_whitespace().count();
        num_words
    }
}
