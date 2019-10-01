#[derive(Default, Clone)]
pub struct CRunArgs {
    pub target: Option<String>,
    pub platform: Option<String>,
    pub extraType: Option<String>,
    pub extra: Option<String>
}
