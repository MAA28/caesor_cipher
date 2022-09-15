mod encode;
mod decode;
mod crack;

fn main() {
    let key = 'z';


    let clear_text = "zurwinterszeitalseinmaleintieferschneelagmussteeinarmerjungehinausgehenundholzaufeinemschlittenholenwieeresnunzusammengesuchtundaufgeladenhattewollteerweilersoerfrorenwarnochnichtnachhausgehensondernerstfeueranmachenundsicheinbisschenwaermendascharrteerdenschneewegundwieersodenerdbodenaufraeumtefandereinenkleinengoldenenschluesselnunglaubteerwoderschluesselwaeremuessteauchdasschlossdazuseingrubindererdeundfandeineiserneskaestchenwennderschluesselnurpasstdachteeressindgewisskostbaresachenindemkaestchenersuchteabereswarkeinschluessellochdaendlichentdeckteereinsabersokleindassmaneskaumsehenkonnteerprobierteundderschluesselpasstegluecklichdadrehteereinmalherumundnunmuessenwirwartenbiservollendsaufgeschlossenunddendeckelaufgemachthatdannwerdenwirerfahrenwasfuerwunderbaresachenindemkaestchenlagen";
    println!("Clear text: {}", clear_text); 

    let ciphered_text = encode::encode(clear_text, key);
    println!("Ciphered text: {}", ciphered_text);

    let deciphered_text = decode::decode(&ciphered_text, key);
    assert_eq!(clear_text, deciphered_text);
    println!("Decoding is working!");

    let cracked_text = crack::crack(&ciphered_text);
    assert_eq!(clear_text, cracked_text);
    println!("Cracking is working!");
}
