use self::homeassistant::HomeAssistant;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use sqlx::{Pool, Postgres};

pub mod homeassistant;
pub mod mail;

pub type SmtpTx = AsyncSmtpTransport<Tokio1Executor>;

/// Shared state used across HTTP handler funcs.
#[derive(Clone)]
pub struct State {
    pub homeassistant: HomeAssistant,
    pub mailer: SmtpTx,
    pub db: Pool<Postgres>,
}
