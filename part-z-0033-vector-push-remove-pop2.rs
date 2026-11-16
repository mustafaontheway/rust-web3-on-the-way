fn main() {

    let ages = vec![37u8, 11, 86, 25, 7, 17, 54, 85];

    let mut can_vote: Vec<bool> = Vec::new();

    for age in ages {

        if age >= 18 && age < 90 {

            can_vote.push(true);

        } else {

            can_vote.push(false);
        }
    }

    println!("Can he/she vote: {can_vote:?}")
}

// Can he/she vote: [true, false, true, true, false, false, true, true]
