#[derive(Default)]
pub struct AuthState {
    pub password: String,
    pub to_be_submitted: bool,
    pub failed_attempts: u16,
}
