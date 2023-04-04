


fn main(){
    mainloop();
}
fn encode(input:String){
    
    let out:String = input.chars()
    .map(|x| match x { 
        'a' => '1', 
        'b'=> '2',
        'c'=> '3',
        'd'=> '4',
        'e'=> '5',
        'f'=> '6',
        'g'=> '7',
        'h'=> '8',
        'i'=> '9',
        'j'=> '0',
        'k'=> 'a',
        'l'=> 'b',
        'm'=> 'c',
        'n'=> 'd',
        'o'=> 'e',
        'p'=> 'f',
        'q'=> 'g',
        'r'=> 'h',
        's'=> 'i',
        't'=> 'j',
        'u'=> 'k',
        'v'=> 'l',
        'w'=> 'm',
        'x'=> 'n',
        'y'=> 'o',
        'z' => '$',
        '1'=> 'p',
        '2'=> 'q',
        '3'=> 'r',
        '4'=> 's',
        '5'=> 't',
        '6'=> 'u',
        '7'=> 'v',
        '8'=> 'w',
        '9'=> 'x',
        '0'=> 'y',

        _ => x
    }).collect();
    println!("Encoded: {}", out);
}

fn decode(input:String){
    let out:String = input.chars()
    .map(|x| match x { 
        '1' => 'a', 
        '2'=> 'b',
        '3'=> 'c',
        '4'=> 'd',
        '5'=> 'e',
        '6'=> 'f',
        '7'=> 'g',
        '8'=> 'h',
        '9'=> 'i',
        '0'=> 'j',
        'a'=> 'k',
        'b'=> 'l',
        'c'=> 'm',
        'd'=> 'n',
        'e'=> 'o',
        'f'=> 'p',
        'g'=> 'q',
        'h'=> 'r',
        'i'=> 's',
        'j'=> 't',
        'k'=> 'u',
        'l'=> 'v',
        'm'=> 'w',
        'n'=> 'x',
        'o'=> 'y',
        '$' => 'z',
        'p'=> '1',
        'q'=> '2',
        'r'=> '3',
        's'=> '4',
        't'=> '5',
        'u'=> '6',
        'v'=> '7',
        'w'=> '8',
        'x'=> '9',
        'y'=> '0',

        _ => x
        
    }).collect();
    println!("decoded: {}", out);
}



fn mainloop(){
    clearscreen::clear().expect("failed to clear screen");
    loop{
        println!("Do you want to use the program? Y/N");
        use console::Term;
        let stdout = Term::buffered_stdout();
        if let Ok(character) = stdout.read_char() {
            match character {
                'y' => choice(),
                'n' => break,
                _ => println!("Not a valid key"),
            }
        }
    }
}
fn choice(){
    clearscreen::clear().expect("failed to clear screen");
    loop{
        println!("Decode or encode? D/E, Q to quit. R to clear");
        use console::Term;
        let stdout = Term::buffered_stdout();
        if let Ok(character) = stdout.read_char() {
            match character {
                'd' => runcode(0),
                'e' => runcode(1),
                'q' => break clearscreen::clear().expect("failed to clear screen"),
                'r' => clearscreen::clear().expect("failed to clear screen"),
                _ => println!("Not a valid key"),
            }
        }
    }
}
fn runcode(input:i32){
    use std::io::{stdin};
    let mut sin=String::new();
    let mut sin2=String::new();
    if input == 0{
        clearscreen::clear().expect("failed to clear screen");
        println!("Decode ->");
        stdin().read_line(&mut sin).expect("Did not enter a correct string");
        decode(sin);
    }
    if input == 1{
        clearscreen::clear().expect("failed to clear screen");
        println!("Encode ->");
        stdin().read_line(&mut sin2).expect("Did not enter a correct string");
        encode(sin2);
    }

    
}
