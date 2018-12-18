#![recursion_limit="128"]
#![feature(generators)]

use eff::*;

struct Hello;
impl Effect for Hello {
    type Output = String;
}

struct World;
impl Effect for World {
    type Output = String;
}

struct Ask {
    prompt: String
}
impl Effect for Ask {
    type Output = String;
}

fn main() {
    let with_effect = eff! {
        let hello = perform!(Hello);
        let name = perform!(Ask {
            prompt: "What's your name?".into()
        });

        format!("{} {}!", hello, name)
    };

    use std::io::{stdin, stdout, Write};
    let stdin = stdin();
    run(with_effect, |x| println!("{}", x), handler! {
        H @ Hello[_] => {
            resume!("Hello".into());
        },
        A @ Ask[Ask { prompt }] => {
            print!("{} ", prompt);
            stdout().flush().unwrap();

            let mut name = String::new();
            match stdin.read_line(&mut name) {
                Ok(_) => resume!(name.trim().into()),
                Err(_) => println!("failed to read"),
            }
        }
    });
}

/*
fn main() {
    let with_effect = eff! {
        let hello = perform!(Hello);
        let world = perform!(World);

        format!("{} {}!", hello, world)
    };

    run(with_effect, |x| println!("{}", x), handler! {
        H @ Hello[_] => {
            resume!("Hello".into());
        },
        W @ World[_] => {
            resume!("World".into());
        }
    });
}
*/
