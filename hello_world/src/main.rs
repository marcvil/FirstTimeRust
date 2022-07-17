use std::io;


fn main() {

    let mut working = true;

    while working{
        println!("Input number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to readline");

        print!("You entered {}", guess);

        if guess.trim() == "0"
        {
            working = false;
        }
    }

}