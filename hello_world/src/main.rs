use std::io;
mod queue;
use queue::Queue;
mod stack;
use stack::Stack;

use std::collections::HashMap;



fn stackexample() {
    let mut stack: Stack<i32> = Stack::new();

    println!("Send a number to stack");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number1: i32 = input1.trim().parse().expect("Please enter a valid number");
    stack.push(number1);

    println!("Send a number to stack");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let number2: i32 = input2.trim().parse().expect("Please enter a valid number");
    stack.push(number2);

    println!("{:?}", stack); 

    /*
    stack.push(1);
    let item = stack.pop();
    assert_eq!(item.unwrap(), 1);
     */
}

fn queueexample() {
    let mut queue: Queue<i32> = Queue::new();

    println!("Send a number to queue");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number1: i32 = input1.trim().parse().expect("Please enter a valid number");
    queue.enqueue(number1);

    println!("Send another number to queue");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let number2: i32 = input2.trim().parse().expect("Please enter a valid number");
    queue.enqueue(number2);

    println!("Queue peek: {:?}", queue.peek());
    println!("Queue dequeue: {:?}", queue.dequeue());
    println!("Queue dequeue: {:?}", queue.dequeue());
}

fn hashmapexample() {
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
    "Grimms' Fairy Tales".to_string(),
    "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                book_reviews.len());
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed.")
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"");
    }

}

fn main() {
    println!("Bienvenidx a mi primer sistema en RUST. Por favor selecciona una opción:");
    println!("1) STACK");
    println!("2) QUEUE");
    println!("3) HASH");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num_input: i32 = input.trim().parse().expect("Please enter a valid number");

    if num_input == 1 {
        println!("Has elegido: STACK");
        stackexample();
    }
    else if num_input == 2 {
        println!("Has elegido: QUEUE");
        queueexample();
    }
    else if num_input == 3 {
        println!("Has elegido: HASH");
        hashmapexample();
    } else {
        println!("Has elegido: {}",input);
    }


}
