// use colored::Colorize;

mod crack;
mod decode;
mod encode;

mod app;

mod tests;

use clap::Parser;

use crate::{encode::encode, decode::decode, crack::crack};

fn main() {
    let args = app::Cli::parse();
    

    match args.command {
        app::Commands::Encode(app::EncodeOptions { text, key }) => {
            if args.verbose {
                println!("Encode \"{}\" with the key \"{}\":", &text, &key);
            }
            let encoded = encode(&text, &key);
            println!("{}", encoded);
        },
        app::Commands::Decode(app::DecodeOptions { text, key }) => {
            if args.verbose {
                println!("Decoding \"{}\" with the key \"{}\"", &text, &key);
            }
            let decoded = decode(&text, &key);
            println!("{}", decoded);
        },
        app::Commands::Crack(app::CrackOptions { text }) => {
            if args.verbose {
               println!("Cracked \"{}\":", &text);
            }
            let cracked = crack(&text);
            println!("{}", cracked);
        }
    }
}

// mod app;

// fn main() {
//     let mode = args().nth(1).expect("No mode given (encode, decode, crack, test)");
//     assert!(mode == "encode" || mode == "decode" || mode == "crack" || mode == "test");
//     if mode != "test" {
//         let text = args().nth(2).expect("No text given");
//         if mode != "crack"  {
//             let key = args().nth(3).expect("No key was given").chars().nth(0).expect("The key should be one letter [a-z]");
//             if mode == "encode" {
//                 println!("Encoding \"{}\" with the key \"{}\"",
//                          text.bold(),
//                          key.to_string().bold());
//                 let encoded_text = encode::encode(&text, key);
//                 println!("{}", encoded_text.bold());
//             } else if mode == "decode" {
//                 println!("Decoding \"{}\" with the key \"{}\"",
//                          text.bold(),
//                          key.to_string().bold());
//                 let decoded_text = decode::decode(&text, key);
//                 println!("{}", decoded_text.bold());
//             }
//         } else {
//             println!("Cracking \"{}\"",
//                 text.bold());

//             let cracked_text = crack::crack(&text);
//             println!("{}", cracked_text.bold());
//         }

//     } else {

//     let key = 'z';

//     let clear_text = "zurwinterszeitalseinmaleintieferschneelagmussteeinarmerjungehinausgehenundholzaufeinemschlittenholenwieeresnunzusammengesuchtundaufgeladenhattewollteerweilersoerfrorenwarnochnichtnachhausgehensondernerstfeueranmachenundsicheinbisschenwaermendascharrteerdenschneewegundwieersodenerdbodenaufraeumtefandereinenkleinengoldenenschluesselnunglaubteerwoderschluesselwaeremuessteauchdasschlossdazuseingrubindererdeundfandeineiserneskaestchenwennderschluesselnurpasstdachteeressindgewisskostbaresachenindemkaestchenersuchteabereswarkeinschluessellochdaendlichentdeckteereinsabersokleindassmaneskaumsehenkonnteerprobierteundderschluesselpasstegluecklichdadrehteereinmalherumundnunmuessenwirwartenbiservollendsaufgeschlossenunddendeckelaufgemachthatdannwerdenwirerfahrenwasfuerwunderbaresachenindemkaestchenlagen";
//     println!("Clear text: {}", clear_text);

//     let ciphered_text = encode::encode(clear_text, key);
//     println!("Ciphered text: {}", ciphered_text);

//     let deciphered_text = decode::decode(&ciphered_text, key);
//     assert_eq!(clear_text, deciphered_text);
//     println!("Decoding is working!");

//     let cracked_text = crack::crack(&ciphered_text);
//     assert_eq!(clear_text, cracked_text);
//     println!("Cracking is working!");
//     }
// }
