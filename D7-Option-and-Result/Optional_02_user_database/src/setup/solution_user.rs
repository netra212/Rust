/* ========================================================== */
/*           🦀 USER STRUCT AND STATUS ENUM 🦀                */
/* ========================================================== */

// TODO 1: Define Status enum with variants: Active, Inactive, Suspended
// Look up enum definition in Rust documentation
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Suspended,
}

// TODO 2: Define User struct with the following fields:
// - id: u32
// - name: String
// - email: String
// - status: Status
// Look up struct definition in Rust documentation
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub status: Status,
}

impl User {
    // TODO 3: Implement new() function to create a new User
    // Create a new user with the given id, name, and email
    // All new users should start with Active status
    // Look up struct initialization in Rust documentation
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            status: Status::Active,
        }
    }

    pub fn get_name(&self) -> &str {
        // TODO: Return a reference to the user's name
        &self.name
    }

    pub fn get_status(&self) -> &Status {
        // TODO: Return a reference to the user's status
        &self.status
    }

    // TODO 4: Implement is_active() method
    // Return true only if the user's status is Active
    // Look up comparing enum variants in Rust documentation
    pub fn is_active(&self) -> bool {
        matches!(self.status, Status::Active)
    }

    // TODO 5: Implement suspend() method
    // Update the user's status to Suspended
    // Look up mutable methods and modifying fields in Rust documentation
    pub fn suspend(&mut self) {
        self.status = Status::Suspended;
    }
}
