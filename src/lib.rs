

pub fn pig_latin(s: &str) -> String {
    const CONSONANTS: &str = "bcdfghjklmnpqrstvwxz";
    // const vowels: &str = "aeiouy";
    // let mut res = "";
    let first_char = s.chars().nth(0);
    if let Some(c) = first_char {
        if CONSONANTS.contains(c) {
            
            format!("{}-{}{}", &s[1..],  c, "ay")
        } else {
            return format!("{}-{}", s, "hay");
        }
    } else {
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin_test() {
        let s1 = "apple";
        let s2 = "first";

        let res = pig_latin(s1);
        assert_eq!(res, "apple-hay");

        let res = pig_latin(s2);
        assert_eq!(res, "irst-fay");
    }
}
