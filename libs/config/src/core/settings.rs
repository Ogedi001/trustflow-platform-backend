//! Settings trait for generic configuration loading
//!
//! Provides a trait for types that can be loaded from configuration sources.

use crate::core::environment::Environment;
use crate::core::error::ConfigResult;
use crate::loader::ConfigLoader;

/// Trait for types that can be loaded from configuration sources
///
/// variables, configuration files, or other sources.

pub trait Settings: Sized + Send + Sync {
    /// Load settings from configuration sources
    fn load(loader: &ConfigLoader) -> ConfigResult<Self>;

    /// Validate the loaded settings
    fn validate(&self, _env: &Environment) -> ConfigResult<()> {
        Ok(())
    }

    fn from_loader(loader: ConfigLoader) -> ConfigResult<Self> {
        let env = loader.environment();
        let settings = Self::load(&loader)?;
        settings.validate(&env)?;
        Ok(settings)
    }
}
