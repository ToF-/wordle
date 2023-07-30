fn main() {
    println!("Hello, world!");
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    Absent,
    Misplaced,
    Correct,
}

type Letter = (char, Status);

pub fn play(trial: &String, secret: &String) -> Vec<Letter> {
    let mut result:Vec<Letter> = trial.chars().map(|c| (c, Status::Absent) ).collect();
    for (i, c) in trial.chars().enumerate() {
        if let Some(p) = secret.find(c) {
            if i == p {
                result[i] = (c, Status::Correct)
            } else {
                result[i] = (c, Status::Misplaced)
            }
        }
    }
    result.sort_by(|(a,_), (b,_)|  a.cmp(b));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_no_letters() {
        let trial: String = String::from("READY");
        let secret: String =String::from("BINGO");
        assert_eq!(play(&trial, &secret), 
                   vec![('A', Status::Absent),
                        ('D', Status::Absent),
                        ('E', Status::Absent),
                        ('R', Status::Absent),
                        ('Y', Status::Absent)])
    }

    #[test]
    fn found_one_misplaced_letter() {
        let trial: String = String::from("GREAT");
        let secret: String =String::from("BINGO");
        assert_eq!(play(&trial, &secret), 
                   vec![('A', Status::Absent),
                        ('E', Status::Absent),
                        ('G', Status::Misplaced),
                        ('R', Status::Absent),
                        ('T', Status::Absent)])
    }
    #[test]
    fn found_one_correct_letter() {
        let trial: String = String::from("READY");
        let secret: String =String::from("BINGY");
        assert_eq!(play(&trial, &secret), 
                   vec![('A', Status::Absent),
                        ('D', Status::Absent),
                        ('E', Status::Absent),
                        ('R', Status::Absent),
                        ('Y', Status::Correct)])
    }
}
