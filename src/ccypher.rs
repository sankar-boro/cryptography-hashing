const ALPHABET: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const LEN: usize = ALPHABET.len();

pub fn encrypt(plain_text: &str, key: usize) -> String {
    let aa: Vec<char> = ALPHABET.chars().collect();
    plain_text
    .chars()
    .map(|c| {
        match ALPHABET.find(c) {
            Some(d) => {
                let e = d + key % LEN;
                let f = aa[e];
                return f;
            },
            None => ' '
        }
    })
    .collect()
}

pub fn decrypt(plain_text: &str, key: usize) -> String {
    let aa: Vec<char> = ALPHABET.chars().collect();

    plain_text
    .chars()
    .enumerate()
    .map(|(_, c)| {
        match ALPHABET.find(c) {
            Some(d) => {
                if key > d {
                    let e = LEN - (key - d);
                    let g = aa[e];
                    return g;
                } else {
                    let e = d - key;
                    let g = aa[e];
                    return g;
                }
            },
            None => ' '
        }
    })
    .collect()
}

pub fn crack(plain_text: &str) {
    let aa: Vec<char> = ALPHABET.chars().collect();
    let mut key: usize = 0;
    loop {
        let d: String = plain_text
        .chars()
        .enumerate()
        .map(|(_, c)| {
            match ALPHABET.find(c) {
                Some(d) => {
                    if key > d {
                        let e = LEN - (key - d);
                        let g = aa[e];
                        return g;
                    } else {
                        let e = d - key;
                        let g = aa[e];
                        return g;
                    }
                },
                None => ' '
            }
        })
        .collect();
        println!("{}", d);
        key += 1;
        if key == 56 { break; }
    }
}