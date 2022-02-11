const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn encrypt(plain_text: &str, key: usize) -> String {
    let aa: Vec<char> = ALPHABET.chars().collect();
    plain_text
    .chars()
    .map(|c| {
        match ALPHABET.find(c) {
            Some(d) => {
                let e = d + key % 52;
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
                    let e = 52 - (key - d);
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