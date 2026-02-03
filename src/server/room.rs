use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Room {
    pub name: String,
    pub password: Option<String>,
    pub members: Vec<SocketAddr>
}


impl Room {
    pub fn new(name: String, password: Option<String>) -> Self {
        Self {
            name,
            password,
            members: Vec::new(),
        }
    }

    pub fn is_password_protected(&self) -> bool {
        self.password.is_some()
    }

    pub fn verify_password(&self, password: &str) -> bool {
        match &self.password {
            Some(pwd) => pwd == password,
            None => true,
        }
    }

    pub fn add_member(&mut self, addr: SocketAddr) {
        if !self.members.contains(&addr) {
            self.members.push(addr)
        }
    }

    pub fn remove_member(&mut self, addr: &SocketAddr) {
        self.members.retain(|&a| a != *addr);
    }
}





