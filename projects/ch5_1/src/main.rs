struct User{
    username: String,
    email : String,
    active: bool,
}

fn main() {
    println!("*** Ch5 ***");


    let mut mohrez = User{
        username : String::from("mohrez"),
        email : String::from("mohrez@gmail.com"),
        active : true,
    };

    println!("user email is {}", mohrez.email);
    


    let mohrez2 = User {
        email: String::from("mohrez2@gmail.com"),
        ..mohrez
    };
    println!("user2 email {}", mohrez2.email);


    // change email

    mohrez.email = String::from("mohammad@gmail.com");

    println!("user email changed to {}", mohrez.email);

}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
    }
}