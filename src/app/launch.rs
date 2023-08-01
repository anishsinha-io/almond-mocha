use derive_more::Display;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum LaunchMode {
    #[display(fmt = "development")]
    Development,
    #[display(fmt = "testing")]
    Testing,
    #[display(fmt = "staging")]
    Staging,
    #[display(fmt = "production")]
    Production,
}
