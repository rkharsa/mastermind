use ansi_term::Colour::Red;
use std::{fmt, io};
use ansi_term::Colour::Green;
use crate::Color::NoColor;

#[derive(Debug)]
#[derive(PartialEq)]

enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Purple,
    Black,
    White,
    NoColor
}

struct ColorDisplay{
    color : String,
    text: String
}
enum CustomError {
    ColorNotInEnum,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    println!("Guess the color!");
    let secret_color = vec![Color::Red,Color::Orange,Color::Yellow,Color::White,Color::NoColor];
    println!("{:?}",fancy_print(&secret_color));
    let mut number_of_guess =0;
    loop{
        println!("Please input your guess. round {}",number_of_guess);
        let foo = String::new();
        let mut guess = foo;
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let colors =str_to_vec_colors(guess);
        print!("{}",invalid_format(&*colors));
        if !invalid_format(&*colors) {
            println!("je suis la ");
            number_of_guess+=1;
            println!("{:?}",fancy_print(&colors));
            println!("{}",
                     Green.paint(
                         format!("{} {}","Number of well placed",
                                 number_of_well_placed_pawns(&secret_color,
                                                             &colors))).to_string()
            );
            println!("{}",Red.paint(format!("{} {}","Number of not well placed",
                                            number_of_not_well_placed_pawns(&secret_color,&colors))).to_string()
            );
            if found_suit(colors, &secret_color){
                println!("{}",Green.paint("You won!"));
                break;
            }

        }
    }
}

/*fn random_secret()-> Vec<Color>{
    let mut rng = thread_rng();
    let mut y = [Color::Red, Color::Green, Color::White, Color::Orange];
    y.
}*/

fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
   return guess.len() as i32-number_of_well_placed_pawns(secret,guess);
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32{
    let mut number_of_well_placed =0;
    for (i,c) in secret.iter().enumerate() {
        if *c==guess[i] {
            number_of_well_placed+=1;
        }
    }
    return number_of_well_placed;
}

fn str_to_vec_colors(guess:String) ->Vec<Color>{
    println!("{}",guess);
    let mut colors:Vec<Color>=Vec::new();
    for  c in guess.trim().chars() {
        println!("{}",c);
        colors.push(match c {
            'R'=>Color::Red,
            'O'=>Color::Orange ,
            'Y'=>Color::Yellow,
            'B'=>Color::Black,
            'W'=>Color::White,
            'G'=>Color::Green,
            'P'=>Color::Purple,
            'I'=>Color::Indigo,
            _ => {
                println!("{}",Red.paint(format!("{} {}",c,"not exist in enum Color")).to_string());
                Color::NoColor;
                break;
            }
        });
    }
    colors
}

fn found_suit( colors:Vec<Color>,secret:&Vec<Color>)->bool{
    let mut good =0;
    for (index,color) in colors.iter().enumerate() {
        if *color==secret[index] {
            good+=1;
        }else{
            return false
        }
    }
    return if good == secret.len() {
        true
    } else {
        false
    }
}

fn invalid_format(guess:&[Color]) ->bool{
    println!("{:?}",fancy_print(&guess));
    if guess.iter().any(|i| *i==Color::NoColor){
        return true
    }
    return false
}

fn fancy_print(guess:&[Color]) -> String {
    let mut str:String="".to_owned();
    for n in guess {
       str.push_str(match n {
           Color::Red => "R",
           Color::Orange =>"O",
           Color::Yellow=>"Y",
           Color::Black=>"B",
           Color::White=>"W",
           Color::Green=>"G",
           Color::Purple=>"P",
           Color::Indigo=>"I",
           Color::Blue=>"B",
           _ => {
               println!("{}",Red.paint(format!("{} {}",n,"not exist in enum Color")).to_string());
               ""
           }
       });
    }
    return str;
}