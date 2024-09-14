use inquire::{MultiSelect, Confirm, Text};
use rand::Rng;

fn main() {
    const PASSWORD_NUM: &str = "1234567890";
    const PASSWORD_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
    const PASSWORD_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const PASSWORD_SYMBOL: &str = "!@#$%^&*()_+-=[]{}|;:',.<>?";

    let requested_len = Text::new("Cipher length:").prompt();
    let mut length: i32 = 8;
    let mut charset: String = String::from("");

    let num_presented = Confirm::new("Do you want integers in cipher?").with_default(true).prompt();

    let options = vec!["lowercase", "uppercase", "symbol"];
    let requested_chars = MultiSelect::new("Choose what kind of characters to include:", options).prompt();

    match requested_len {
        Ok(requested_len) => match requested_len.parse::<i32>() {
            Ok(num) => {
                println!("Cipher length is: {}", num);
                length = num;
            }
            Err(_) => {
                println!("Input should be an integer");
            }
        },
        Err(_) => {
            println!("Please input an integer");
        }
    }

    match num_presented {
        Ok(num_presented) => {
            if num_presented {
                println!("Integers will be included");
                charset.push_str(PASSWORD_NUM)
            } else {
                println!("Integers will NOT be included");
            }
        }
        Err(_) => println!("Choose if you want integers in your cipher")
    }

    match requested_chars {
        Ok(choices) => {
            for choice in choices {
                match choice {
                    "lowercase" => charset.push_str(PASSWORD_LOWER),
                    "uppercase" => charset.push_str(PASSWORD_UPPER),
                    "symbol" => charset.push_str(PASSWORD_SYMBOL),
                    _ => (),
                }
            }
        }
        Err(_) => println!("Wrong choices")
   }

   let mut result = gen_password(&charset, length);

    match num_presented {
        Ok(presented) => {
            if presented {
                loop {
                    let original_res = gen_password(&charset, length);
                    if original_res.chars().any(|char| char.is_digit(10)) {
                        result = original_res;
                        break;
                    }
                }
            }
        }
        Err(_) => {}
    }

   println!("Your password is: {:?}", result)
}

fn gen_password(charset: &str, length: i32) -> String {
    let mut rng = rand::thread_rng();
    let char_bytes: &[u8] = charset.as_bytes();
    let result= (0..length).map(|_| {
        let idx = rng.gen_range(0..char_bytes.len());
        char_bytes[idx] as char
    }).collect();

    result
}
