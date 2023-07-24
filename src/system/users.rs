use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    uid: u32,
    groups: Vec<String>,
}

impl From<users::User> for User {
    fn from(value: users::User) -> Self {
        Self {
            name: value.name().to_string_lossy().to_string(),
            uid: value.uid(),
            groups: value
                .groups()
                .unwrap()
                .to_vec()
                .iter()
                .map(|x| x.name().to_string_lossy().to_string())
                .collect(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "user_default".to_string(),
            uid: 1000,
            groups: vec!["group1".to_string(), "group2".to_string()],
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "User Name : {}", self.name)?;
        writeln!(f, "UID : {}", self.uid)?;
        if self.groups.is_empty() {
            writeln!(f, "This user is not in any groups")?;
        } else {
            writeln!(f, "User Groups : ")?;
            self.groups.clone().iter().for_each(|group| {
                writeln!(f, "{}", group).unwrap();
            });
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>,
}

impl Users {
    pub fn new() -> Self {
        let iter = unsafe { users::all_users() };
        let mut vec_users = Vec::new();
        for user in iter {
            let user_iter: User = user.into();
            vec_users.push(user_iter);
        }
        Self { users: vec_users }
    }
    pub fn find_one_user_by_name(&self, name: &str) -> Option<User> {
        for user in &self.users {
            if user.name.to_lowercase() == name.to_lowercase() {
                return Some(user.clone());
            }
        }
        None
    }
    pub fn find_one_user_by_uid(&self, user_uid: u32) -> Option<User> {
        for user in &self.users {
            if user.uid == user_uid {
                return Some(user.clone());
            }
        }
        None
    }
}
impl Default for Users {
    fn default() -> Self {
        let vec_users = vec![User::default(), User::default()];
        Self { users: vec_users }
    }
}

impl fmt::Display for Users {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.users.iter().for_each(|user| {
            writeln!(f, "User : {}", user).unwrap();
        });
        Ok(())
    }
}
