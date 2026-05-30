/* ========================================================== */
/*           🦀 USER STRUCT AND STATUS ENUM 🦀                */
/* ========================================================== */

// TODO 1: Define Status enum with variants: Active, Inactive, Suspended
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Suspended,
}

// TODO 2: Define User struct with fields: id, name, email, status
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub status: Status,
}

impl User {
    // TODO 3: Implement new() — id, name, email; new users start with Active status
    pub fn new(_id: u32, _name: &str, _email: &str) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            status: Status::Active,
        }
    }

    pub fn get_name(&self) -> &str {
        // TODO: Return a reference to the user's name
        ""
    }

    pub fn get_status(&self) -> &Status {
        // TODO: Return a reference to the user's status
        &Status::Active
    }

    // TODO 4: Implement is_active() — true only if status is Active
    pub fn is_active(&self) -> bool {
        false
    }

    // TODO 5: Implement suspend() — set status to Suspended
    pub fn suspend(&mut self) {
        // TODO: Change status to Suspended
    }
}

// ============================================================
// 💡 HINTS (read only if stuck)
// ============================================================
//
// new():
//   - Initialize all fields with the given parameters; convert &str with .to_string()
//   - Status should be Status::Active by default
//   - Look up: struct initialization, to_string()
//
// get_name / get_status:
//   - Return &self.name and &self.status respectively
//   - Look up: returning references from &self methods
//
// is_active:
//   - Compare self.status == Status::Active
//   - Look up: PartialEq, == on enum variants
//
// suspend:
//   - Assign Status::Suspended to self.status
//   - Look up: &mut self, field assignment
