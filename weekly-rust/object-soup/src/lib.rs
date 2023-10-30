//
// class Person:
//     def __init__(self, name):
//          self.name = name
//          self.friends = []

// def add_friend(self, other):
//      self.friends.append(other)
//
// alice = Person("Alice")
// bob = Person("Bob")
// alice.add_friend(bob)
// bob.add_friend(alice)

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub friends: Vec<Person>,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.into(),
            friends: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, other: Person) {
        self.friends.push(other);
    }
}

#[test]
#[ignore = "value borrowed here after move"]
fn test_person() {
    let mut alice = Person::new("Alice");
    let mut bob = Person::new("Bob");
    alice.add_friend(bob.clone());
    bob.add_friend(alice); // error: borrow of moved value: `bob`
                           // value borrowed here after move
}
