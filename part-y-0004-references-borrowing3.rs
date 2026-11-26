fn main() {

    let members: (String, String, String) = ("Ayhan".to_string(), "Aygün".to_string(), "Ayben".to_string());

    //let second_member = members.1;

    //println!("{:?}", members); // error[E0382]: borrow of partially moved value: `members` 

    let _second_member = &members.1;

    println!("{:?}", members);
}

