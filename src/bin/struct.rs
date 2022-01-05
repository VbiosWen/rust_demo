#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

impl User {
    fn area(&mut self) {
        self.username.push_str("test")
    }

    fn active_check(&self) -> &str {
        if self.active {
            return "active";
        }
        "no active"
    }
}


fn main() {
    let mut user1 = User {
        email: String::from("tengmu@dian.so"),
        username: String::from("tengmu"),
        sign_in_count: 100,
        active: true,
    };
    user1.username = String::from("tengmu1");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("tengmu1@dian.so"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{:#?}", user2);

    let user3 = User {
        email: String::from("tengmu3@dian.so"),
        ..user2
    };

    println!("{:#?}", user3);
}