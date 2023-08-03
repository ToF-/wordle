use ansi_term::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    Unknown,
    Absent,
    Misplaced,
    Correct,
}

type Letter = (char, Status);

pub fn play(trial: &String, secret: &String) -> Vec<Letter> {
    let mut result:Vec<Letter> = trial.chars().map(|c| (c, Status::Absent) ).collect();
    let mut checked:Vec<bool> = vec![false, false, false, false, false];
    // first look for all correct letters
    for (i, c) in trial.chars().enumerate() {
        if let Some(ch) = secret.chars().nth(i) { 
            if c == ch {
                result[i] = (c, Status::Correct);
                checked[i] = true;
            }
        }
    }
    // then look for all misplaced letters
    for (i, c) in trial.chars().enumerate() {
        if !checked[i] {
            for (j, d) in secret.chars().enumerate() {
                if !checked[j] && d == c {
                    result[i] = (c, Status::Misplaced);
                    checked[j] = true;
                }

            }
        }
    }
    result
}

pub fn show_result(result: Vec<Letter>) {
    let mut styles: Vec<Style> = vec![Colour::White.normal();5];
    for (i, (c,s)) in result.iter().enumerate() {
        styles[i] = match s {
            Status::Unknown   => Color::White.normal().dimmed().on(Color::Black),
            Status::Absent    => Color::Blue.normal().dimmed().on(Color::Black),
            Status::Misplaced => Color::White.normal().bold().on(Color::Yellow),
            Status::Correct   => Color::White.normal().bold().on(Color::Green),
        };
        print!("{}",styles[i].paint(c.to_string()));
    }
    println!("");

}
fn main() {
    show_result(play(&"READY".to_string(),&"BINGO".to_string()));
    show_result(play(&"GREAT".to_string(),&"BINGO".to_string()));
    show_result(play(&"SMALL".to_string(),&"GRAAL".to_string()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_no_letters() {
        let trial: String = String::from("READY");
        let secret: String =String::from("BINGO");
        assert_eq!(play(&trial, &secret), 
                   vec![('R', Status::Absent),
                        ('E', Status::Absent),
                        ('A', Status::Absent),
                        ('D', Status::Absent),
                        ('Y', Status::Absent)])
    }

    #[test]
    fn found_one_misplaced_letter() {
        let trial: String = String::from("GREAT");
        let secret: String =String::from("BINGO");
        assert_eq!(play(&trial, &secret), 
                   vec![('G', Status::Misplaced),
                        ('R', Status::Absent),
                        ('E', Status::Absent),
                        ('A', Status::Absent),
                        ('T', Status::Absent)])
    }
    #[test]
    fn found_one_correct_letter() {
        let trial: String = String::from("READY");
        let secret: String =String::from("BINGY");
        assert_eq!(play(&trial, &secret), 
                   vec![('R', Status::Absent),
                        ('E', Status::Absent),
                        ('A', Status::Absent),
                        ('D', Status::Absent),
                        ('Y', Status::Correct)])
    }
    #[test]
    fn found_one_correct_and_one_misplaced_letters() {
        let trial: String = String::from("SMALL");
        let secret: String =String::from("GRAAL");
        assert_eq!(play(&trial, &secret), 
                   vec![('S', Status::Absent),
                        ('M', Status::Absent),
                        ('A', Status::Correct),
                        ('L', Status::Absent),
                        ('L', Status::Correct)])
    }

}
