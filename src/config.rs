use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub astarte: AstarteCfg,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AstarteCfg {
    pub appengine: Url,
}
