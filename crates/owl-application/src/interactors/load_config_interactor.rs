use crate::UsecaseError;
use crate::output_ports::load_config_output::LoadConfigOutput;
use anyhow::Result;
use owl_infra::OwlConfig;
use std::path::PathBuf;

pub struct LoadConfigInteractor<'a, P>
where
    P: LoadConfigOutput + Send + 'a + ?Sized,
{
    path: PathBuf,
    presenter: &'a mut P,
}

impl<'a, P> LoadConfigInteractor<'a, P>
where
    P: LoadConfigOutput + Send + 'a + ?Sized,
{
    pub fn new(path: PathBuf, presenter: &'a mut P) -> Self {
        Self { path, presenter }
    }

    pub async fn execute(&mut self) -> Result<OwlConfig, UsecaseError> {
        let data = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(UsecaseError::ConfigReadError)?;
        let config: OwlConfig = toml::from_str(&data).map_err(UsecaseError::ConfigParseError)?;
        self.presenter.on_success(&config);
        Ok(config)
    }
}
