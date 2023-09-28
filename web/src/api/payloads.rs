use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterInput {
    username: String,
    password: String,
    password_conf: String,
    battletag: String
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String
}

/*--------------------------------------------------
 * There is most definitely a better way to do this.
--------------------------------------------------*/
impl RegisterInput {
    pub fn get_username(&self)      -> &str { &self.username      }
    pub fn get_password(&self)      -> &str { &self.password      }
    pub fn get_password_conf(&self) -> &str { &self.password_conf }
    pub fn get_battletag(&self)     -> &str { &self.battletag     }
}

impl LoginInput {
    pub fn get_username(&self)      -> &str { &self.username      }
    pub fn get_password(&self)      -> &str { &self.password      }
}

