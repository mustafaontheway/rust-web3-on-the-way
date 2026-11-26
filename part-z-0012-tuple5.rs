fn main() {

    let members: (String, String, String) = ("Ayhan".to_string(), "Ayben".to_string(), "Aygün".to_string());

    //let second_member = members.2;

    //println!("{:?}", members); // error[E0382]: borrow of partially moved value: `members` 

    let _second_member = &members.2;

    println!("{:?}", members);
}

