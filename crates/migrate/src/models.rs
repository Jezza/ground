use crate::ServiceName;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
    pub metadata: Option<WorkspaceMetadata>,
    pub packages: Vec<Package>,

    pub migration_sources: Vec<MigrationSource>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WorkspaceMetadata {
    #[serde(default, alias = "migration")]
    pub migrations: Vec<ExplicitMigration>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PackageMetadata {
    #[serde(default)]
    pub skip: bool,
    pub schema: Option<String>,
    #[serde(default, alias = "migration")]
    pub migrations: Option<Migrations>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Migrations {
    Flag(bool),
    Source(Source),
    List(Vec<Migration>),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Migration {
    Full(ExplicitMigration),
    Source(Source),
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ExplicitMigration {
    pub source: Source,
    pub schema: String,
}

/// This could be a crate name, a folder, or a file.
pub type Source = String;

#[derive(Debug, Clone)]
pub struct MigrationSource {
    pub source: PathBuf,
    pub schema: Option<ServiceName>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub schema_name: ServiceName,

    pub crate_root: PathBuf,
    pub manifest_path: PathBuf,
    pub metadata: Option<PackageMetadata>,

    pub migration_sources: Vec<MigrationSource>,
}
