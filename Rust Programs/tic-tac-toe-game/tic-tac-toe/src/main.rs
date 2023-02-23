/* ----------------- Text based Tic Tac Toe game ------------- */
/*
    This is a simple text based tic tac toe game coded in Rust

    Info about the game:
    > 2 players will be able to play the game, both sitting at the same
    computer
    > The board will be println!ed out every time the player makes a move
    > The game accepts an input of the player"s position which will then
    place a symbol on the board
*/

/* ----------------------- main ----------------------- */
fn main() {
    // we'll create a hashmap to map int to array element
    let mut hm_choice: HashMap<i32, [i32;2]> = HashMap::new();
    hm_choice.insert(1, [0,0]);
    hm_choice.insert(2, [0,1]);
    hm_choice.insert(3, [0,2]);
    hm_choice.insert(4, [1,0]);
    hm_choice.insert(5, [1,1]);
    hm_choice.insert(6, [1,2]);
    hm_choice.insert(7, [2,0]);
    hm_choice.insert(8, [2,1]);
    hm_choice.insert(9, [2,2]);

    // this will build our empty board 
    let mut board = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    // this will be a counter of how many turns we've had in the game
    let mut turns = 0;
    // this is the max turns the game can have
    let max_turns = 9;
    // players in the game
    let players = ["X", "O"];
    
    // while loop, to loop while the number of turns is less/equal to max_turns
    while turns < max_turns {
        // for loop for numbers of players
        for p in players {
            // we"ll println! the board at the start of every turn
            println!("");
            println!("{:?}", board);
            println!("");
            // break out of for loop if we"ve played all turns
            if turns == max_turns {
                println!("");
                println!("Nobody wins!");
                break;
            } else if turns > max_turns {
                break;
            } else if p == "X" {
                println!("You are player X");
                // user inputs in which element they'd like their input
                let element_choice = choice();
                let element_choice = hm_choice.get(&element_choice).unwrap();
                // this will input the user"s choice on the board
                // it will also take care of wrong inputs
                x_choice(board, element_choice);
                // we"ll increment by one at the end of each turn
                turns+=1;
                // check to see if player x has won
                if won_game(board) == True {
                    println!("");
                    println!("{:?}", board);
                    println!("");
                    println!("Player X has won!");
                    println!("");
                    turns = max_turns + 1;
                    break;
                }
            }
            // player o
            else if p == "O" {
                println!("You are player O");
                // user inputs in which element they"d like their input
                let element_choice = choice();
                let element_choice = hm_choice.get(&element_choice).unwrap();
                // this will input the user"s choice on the board
                // it will also take care of wrong inputs
                o_choice(board, element_choice);
                // we"ll increment by one at the end of each turn
                turns+=1;
                // check to see if player o has won
                if won_game(board) == True {
                    println!("");
                    println!("{:?}", board);
                    println!("");
                    println!("Player O has won!");
                    println!("");
                    turns = max_turns + 1;
                    break;
                }
            }
        }
    }
}

/* ----------------------- packages ----------------------- */
// this will be used by the computer to choose rock paper scissors at random
use ndarray::arr2;
// we'll use this to map the user's input to the element in the array
use std::collections::HashMap;
// this will allow the user to input their choice for the game
use std::io;

/* ----------------------- functions ----------------------- */
// this function will allow the user to choose the element in which they"d like
// to add their input
fn choice() -> i32 {
    // the only values we'll accept for our user input
    let correct_input: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // this is a loop, which will only end if a valid input is entered
    let choice = loop {
        println!("");
        println!("Your Turn, choose an element: ");

        // this is Rust's user input method
		let mut choice = String::new();
		io::stdin().read_line(&mut choice).unwrap();

        // if the value entered was int
		if let Ok(val) = choice.trim().parse::<i32>() {
            if correct_input.contains(&val) {
                break val;
            }
            // this will bring if the value wasn't our valid input values
            println!("");
            println!("Wrong Input!");
            println!("Sorry, but you did not choose a valid element");
			continue;
		}

        // this will be printed if the value entered was not int
        println!("");
        println!("Wrong Input!");
        println!("Sorry, but you did not choose a valid element");
	};
     
    return choice;
}
