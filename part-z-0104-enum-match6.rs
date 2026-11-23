fn main() {

    let r1 = score_results(77);

    println!("{r1:?}"); // Medium(77)

    let r2 = score_results(105);
}

#[derive(Debug)]
enum Scores {

    Best(u8),
    Good(u8),
    Medium(u8),
    Bad(u8)
}

fn score_results(score: u8) -> Scores {

    match score {
        
        90..=100 => Scores::Best(score),
        80..90 => Scores::Good(score),
        70..80 => Scores::Medium(score),
        0..70 => Scores::Bad(score),
        101..=u8::MAX => {
            println!("Score value max value can't exceed 100!");
            todo!();
        }
    }
}

