//! # Role-Based Access Control Submodule (`ferrox-front-security::rbac`)
//!
//! `rbac` provides authorization types (`Role`, `UserIdentity`) and the `<Secure>` DOM Anti-Tampering component.
//!
//! ## Key Components & Types
//! - `Role`: Enum for user roles (`Guest`, `User`, `Admin`).
//! - `UserIdentity`: User credentials container (`id`, `role`, `permissions`, `has_permission()`).
//! - `secure(identity, require, child)`: Mounts child DOM element into memory ONLY if permission check passes.

use ferrox_front_core::reactivity::Signal;
use ferrox_front_core::dom::{div, DomBuilder};

#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    Guest,
    User,
    Admin,
}

#[derive(Clone, Debug)]
pub struct UserIdentity {
    pub id: String,
    pub role: Role,
    pub permissions: Vec<String>,
}

impl UserIdentity {
    pub fn guest() -> Self {
        Self {
            id: "".to_string(),
            role: Role::Guest,
            permissions: vec![],
        }
    }

    pub fn has_permission(&self, perm: &str) -> bool {
        if self.role == Role::Admin {
            return true; // Admin bypasses all permission checks
        }
        self.permissions.contains(&perm.to_string())
    }
}

/// `<Secure>` Component: Mounts child DOM elements into memory ONLY if the user holds required permission.
pub fn secure(identity: &UserIdentity, require: &str, child: DomBuilder) -> DomBuilder {
    if identity.has_permission(require) {
        div().child(child)
    } else {
        // Anti-Tampering: If unauthorized, child nodes are never instantiated in DOM memory
        div().attr("style", "display: none;")
    }
}