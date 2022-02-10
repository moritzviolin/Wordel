use termion::color;
use std::io;
use std::{thread, time};

mod wordlist;

pub use crate::wordlist::wordlist_actions;

fn print_manual() {
    println!("How to play: ------------------------------------------------------------");
    println!("{}Black {}means that the letter does not appear in the solution", color::Fg(color::Black), color::Fg(color::White));
    println!("");
    println!("{}Red {}means that the letter appears at a different position in the solution", color::Fg(color::Red), color::Fg(color::White));
    println!("");
    println!("{}Green {}means that the letter is at the right place", color::Fg(color::Green), color::Fg(color::White));
    println!("");
    println!("Word list:");
    println!("https://github.com/tabatkins/wordle-list");
    println!("-------------------------------------------------------------------------");
    println!("");
    println!("");
    println!("LET THE GUESSING BEGIN!");
    println!("");
}


fn generate_wordlist(answer: &str) -> [char; 5] {
    let mut answered_character_list: [char; 5] = ['a','a','a','a','a'];
    for i in 0.. 5 {
        //put chars of the solution into a list for easier comparison during guess_validation()
        let answered_character = answer.chars().nth(i).unwrap();
        answered_character_list[i] = answered_character
    }
    return answered_character_list
}


fn guessing(guess_nr: i32) -> [char; 5] {
    let guess_number = match guess_nr{
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        _ => "n-th"
    };

    let reader = io::stdin();

    loop {
        //input loop to guarantee that each guess has 5 chars
        let mut guess: String = String::new();
        println!("Enter your {} guess:", guess_number);
        let _result = reader.read_line(&mut guess);
        //remove the newline character added by pressing enter
        guess.pop();
        //println!("len: {}", guess.len()); //<-- comment out for actual build, useful for debugging
        if guess.len() == 5 {
            //validate guess length -> proceed
            //println!("You guessed: {}", guess); //<-- comment out for actual build, useful for debugging
            let mut guessed_character_list: [char; 5] = ['a','a','a','a','a'];
            for i in 0.. 5 {
                //put chars of the guessed word into a list for easier comparison during guess_validation() and easier printout of the validated guess
                let guessed_character = guess.chars().nth(i).unwrap();
                guessed_character_list[i] = guessed_character;
            }
            return guessed_character_list;
        }
        else {
            println!("Your guess has to have 5 characters!");
        }
    }
}


fn guess_validation(guessed_character_list: [char; 5], answered_character_list: [char; 5]) -> Vec<&'static str> {
    let mut guess_working_list = guessed_character_list;
    let mut answer_working_list = answered_character_list;

    let mut coloring = Vec::new();
    let mut green_char_indices = Vec::new();
    let mut red_char_indices = Vec::new();

    //look for all right characters
    for pos in 0..5 {
        if guess_working_list[pos] == answer_working_list[pos] {
            green_char_indices.push(pos);
            guess_working_list[pos] = '0';
            answer_working_list[pos] = '0';
        }
    }
    //look for all misplaced characters
    for pos in 0..5 {
        if char_in_list(answer_working_list, guess_working_list[pos]) {
            red_char_indices.push(pos);
            //removing the characters from the list so multiple occurances of the same character won't cause problems
            guess_working_list[pos] = '0';
            answer_working_list[pos] = '0';
        }
    }
    //setting black characters as a default value
    for _i in 0..5 {
        coloring.push("black")
    }
    //overwriting the colors to match the state of the letter
    for pos in red_char_indices {
        coloring[pos] = "red";
    }
    for pos in green_char_indices {
        coloring[pos] = "green";
    }
    return coloring
}


fn char_in_list(list: [char;5],character: char) -> bool {
    for i in list {
        if i == character {
            return true
        }
    }
    return false
}


fn main() {
    let answer = wordlist_actions::random_word();
    //println!("answer: {}", answer); //<-- comment out for actual build, useful for debugging
    let answer_list = generate_wordlist(answer);
    print_manual();
    for i in 1..=5 {
        let guess = guessing(i);
        let coloring = guess_validation(guess, answer_list);

        for character in 0..5 {
            //chose in which color which character should appear
            match coloring[character] {
                "green" => print!("{}{}", color::Fg(color::Green), guess[character]),
                "red" => print!("{}{}", color::Fg(color::Red), guess[character]),
                "black" => print!("{}{}", color::Fg(color::Black), guess[character]),
                _ => print!("{}{}", color::Fg(color::White), guess[character])
            }
        };

        print!{"\n{}", color::Fg(color::White)};
        if coloring == ["green", "green", "green", "green", "green"] {
            println!("You guessed correctly! Congrats");
            thread::sleep(time::Duration::new(10, 0));
            break;
        }
        if (coloring != ["green", "green", "green", "green", "green"]) & (i == 5) {
            println!("You couldn't find the solution within 5 guesses. The solution was {}. Better luck next time!", answer);
        }
    };

}
