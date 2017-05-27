/*
 Crate:         ore
 File:          /project/member.rs
 Module:        ::project::member
 Visibility:    public
 */

// TODO: documentation

use std::fmt::{Display, Formatter, Result as FmtResult};

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    head_role: Role,
    name: String,
    roles: Vec<Role>,
    user_id: u32,
}

impl Member {

    // TODO: documentation
    pub fn head_role(&self) -> Role {
        self.head_role
    }

    // TODO: documentation
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    // TODO: documentation
    pub fn roles(&self) -> Vec<Role> {
        self.roles.to_vec()
    }

    // TODO: documentation
    pub fn user_id(&self) -> u32 {
        self.user_id
    }
}

// TODO: documentation
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum Role {
    Owner,
    Admin,
    Developer,
    Editor,
    Support,
}

impl Display for Role {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match *self {
            Role::Admin => "Admin",
            Role::Developer => "Developer",
            Role::Editor => "Editor",
            Role::Owner => "Owner",
            Role::Support => "Support",
        })
    }
}
