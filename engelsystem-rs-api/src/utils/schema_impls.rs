use schemars::JsonSchema;
use serde::Deserialize;
use zeroize::{Zeroize, Zeroizing};

#[derive(JsonSchema, Deserialize)]
#[serde(remote = "Zeroizing")]
pub struct ZeroizingDef<Z: Zeroize>(#[serde(getter = "Deref::deref")] Z);

impl<Z: Zeroize> From<ZeroizingDef<Z>> for Zeroizing<Z> {
    fn from(value: ZeroizingDef<Z>) -> Self {
        Zeroizing::new(value.0)
    }
}
