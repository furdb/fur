pub struct Person {
    id: u8,                // Takes 5 bits
    favourite_number: u16, // Takes 11 bits
}

impl Person {
    pub fn new(id: u8, favourite_number: u16) -> Person {
        Person {
            id,
            favourite_number,
        }
    }
}
