use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    Visitor,
}

impl Default for Role {
    fn default() -> Self {
        Self::Visitor
    }
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        use Role::*;
        match role {
            "ADMIN" => Admin,
            "USER" => User,
            "VISITOR" => Visitor,
            _ => panic!("unknown role"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_from() {
        use Role::*;
        assert_eq!(Role::from("ADMIN"), Admin);
        assert_eq!(Role::from("USER"), User);
        assert_eq!(Role::from("VISITOR"), Visitor);
    }
}
