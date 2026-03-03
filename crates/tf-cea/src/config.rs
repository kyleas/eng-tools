use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BackendExecutable {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CeaBackendConfig {
    pub executable: Option<BackendExecutable>,
}

impl CeaBackendConfig {
    pub fn from_env() -> Self {
        let executable =
            std::env::var_os("TF_CEA_BACKEND_EXECUTABLE").map(|path| BackendExecutable {
                path: PathBuf::from(path),
            });
        Self { executable }
    }
}
