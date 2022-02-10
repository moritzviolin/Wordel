use termion::color;
use std::io;


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
}


fn generate_word() -> [char; 5] {
    let answer = wordlist_actions::random_word();
    //println!("{}", answer);

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
        let mut guess: String = String::new();
        println!("Enter your {} guess:", guess_number);
        let _result = reader.read_line(&mut guess);
        guess.pop();
        println!("len: {}", guess.len());
        if guess.len() == 5 {
            println!("You guessed: {}", guess);

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

    let mut coloring = Vec::new();

    //validate the characters (right place, in word, not in word)
    for pos in 0..5 {
        //check, if in right place and recolor
        if guessed_character_list[pos] == answered_character_list[pos] {
            coloring.push("green");
        }
        else {
            //check if in word and recolor
            if char_in_list(answered_character_list, guessed_character_list[pos]) {
                coloring.push("red");
            }
            else {
                coloring.push("black");
            }
        }
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
    let answer = generate_word();
    print_manual();
    for i in 1..=5 {
        let guess = guessing(i);
        let coloring = guess_validation(guess, answer);
        for character in 0..5 {
            match coloring[character] {
                "green" => print!("{}{}", color::Fg(color::Green), guess[character]),
                "red" => print!("{}{}", color::Fg(color::Red), guess[character]),
                "black" => print!("{}{}", color::Fg(color::Black), guess[character]),
                _ => print!("{}{}", color::Fg(color::White), guess[character])
            }
        }
        print!{"\n{}", color::Fg(color::White)};
        if coloring == ["green", "green", "green", "green", "green"] {
            println!("You guessed correctly! Congrats");
            break;
        }
    };
}
