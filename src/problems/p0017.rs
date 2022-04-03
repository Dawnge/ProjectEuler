use std::vec;


pub fn p0017() {
    use std::time::Instant;
    let now = Instant::now();
    let a = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
    let b = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
    let mut all_words: Vec<String> = vec![];
    for word in a {
        all_words.push(String::from(word));
    }
    for word in b {
        all_words.push(String::from(word));
    }
    let c = ["Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
    for word in c {
        all_words.push(String::from(word));
        for word2 in a {
            all_words.push(format!("{}{}", word, word2));
        }
    }

    let d = ["OneHundredAnd", "TwoHundredAnd", "ThreeHundredAnd", "FourHundredAnd", "FiveHundredAnd", "SixHundredAnd", "SevenHundredAnd", "EightHundredAnd", "NineHundredAnd"];

    for word0 in d {
        let mut czars = word0.chars();
        czars.next_back();
        czars.next_back();
        czars.next_back();
        
        all_words.push(String::from(czars.as_str()));
        for word in a {
            all_words.push(format!("{}{}",word0,  word));
        }
        for word in b {
            all_words.push(format!("{}{}",word0,  word));
        }
        for word in c {
            all_words.push(format!("{}{}",word0,  word));
            for word2 in a {
                all_words.push(format!("{}{}{}",word0,  word, word2));
            }
        }
    }
    all_words.push(String::from("OneThousand"));
    
    let mut sol: u32 = 0;
    for word in all_words {
        sol += word.len() as u32;
        //println!("{}", word);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Problem 0017 Solution:");
    println!("{:?}", sol);

}