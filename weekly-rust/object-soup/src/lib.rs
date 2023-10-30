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
pub struct Person<'a> {
    pub name: String,
    pub friends: Vec<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.into(),
            friends: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, other: &'a Person) {
        self.friends.push(other);
    }
}

#[test]
#[ignore = "value borrowed here after move"]
fn test_person() {
    let mut alice = Person::new("Alice");
    let mut bob = Person::new("Bob");
    // alice.add_friend(&bob);
    // bob.add_friend(&alice);

    // error[E0502]: cannot borrow `bob` as mutable because it is also borrowed as immutable
    // --> weekly-rust/object-soup/src/lib.rs:40:5
    // |
    // 39 |     alice.add_friend(&bob);
    // |                      ---- immutable borrow occurs here
    // 40 |     bob.add_friend(&alice); // error: borrow of moved value: `bob`
    // |     ^^^^----------^^^^^^^^
    // |     |   |
    // |     |   immutable borrow later used by call
    // |     mutable borrow occurs here
}
