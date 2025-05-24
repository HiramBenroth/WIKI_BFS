#![allow(dead_code)]


use clearscreen::clear;
/* MENU STRUCTUR */
pub struct Menu<'a> {
    pub name: String,
    pub options: Vec<&'a str>,
}

impl Menu<'_> {
    fn display(&self) {
        let mut cntr = 0;
        println!("\nHere are your {}options:", self.name);

        for option in self.options.iter() {
            println!("{}. {}", cntr, option);
            cntr += 1;
        }
    }

    pub fn get_option(&self) -> i8 {
        use input::get_input_i8;
        self.display();
        let mut select : i8 = -1;
        while  0 > select || select > self.options.len().try_into().unwrap()  {
            select = get_input_i8(format!("Which options do you select  (0 - {}) ?", self.options.len()- 1))
        }
        select
    }
}

// Important Functions
pub fn clear_console() {
        clear().expect("Failled to clear Terminal");
}

// This function returns a String of a input, Should update to be able to get any input.

pub mod input {
    use std::io::{self, Write};

    pub fn get_input_string(prompt : String) -> String {
        {

            print!("{}", prompt);
            io::stdout().flush().unwrap(); 

            let mut input : String = String::new();
            io::stdin()
                .read_line(&mut input).expect("Failed to read");
            
            input.trim().to_string()
        }        
    }


    pub fn get_input_i32(prompt : String) -> i32{
        get_input_string(prompt).parse().expect("Error- not an int")
    }

    pub fn get_input_i16(prompt : String) -> i16{
        get_input_string(prompt).parse().expect("Error- not an int")
    }

    pub fn get_input_i8(prompt : String) -> i8{
        get_input_string(prompt).parse().expect("Error- not an int")
    }

    pub fn get_input_f32(prompt : String) -> f32{
        get_input_string(prompt).parse().expect("Error- not an int")
    }

}