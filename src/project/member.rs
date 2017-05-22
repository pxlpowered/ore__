/*
 Crate:         ore
 File:          /project/member.rs
 Module:        ::project::member
 Visibility:    public
 */

// TODO: documentation

// TODO: documentation
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member<'a> {
    head_role: Role,
    name: &'a str,
    roles: Vec<Role>,
    user_id: u32,
}

// TODO: documentation
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(u8)]
pub enum Role {
    Admin,
    Developer,
    Editor,
    Owner,
    Support,
}
