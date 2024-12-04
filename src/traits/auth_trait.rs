use passwords_rs::PasswordHash;
use rust_iam::{EngineTrait, PolicyCollection};

pub trait AuthTrait<Engine: EngineTrait> {
    fn set_password<const BACKEND: u8>(password: PasswordHash);
    fn password(&self) -> &PasswordHash;
    fn save() -> Result<bool, &'static str>;
    fn policies(&self) -> &PolicyCollection<Engine>;
}
