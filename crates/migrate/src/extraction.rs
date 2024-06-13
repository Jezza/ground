use std::path::{Path, PathBuf};

use crate::models::{ExplicitMigration, Migration, MigrationSource, Migrations, Package};
use crate::{models, ServiceName};
use anyhow::Context as _;
use models::Source;

pub fn extract_workspace() -> anyhow::Result<models::Workspace> {
    extract_with_config(false)
}

pub fn extract() -> anyhow::Result<models::Workspace> {
    extract_with_config(true)
}

const CARGO_TOML: &str = "Cargo.toml";

fn manifests(
    cwd: &Path,
) -> impl Iterator<Item = anyhow::Result<(PathBuf, cargo_toml::Manifest)>> + '_ {
    cwd.ancestors()
        .filter_map(|path| {
            let manifest = path.join(CARGO_TOML);
            manifest.exists().then_some(manifest)
        })
        .map(|path| {
            let manifest = cargo_toml::Manifest::from_path(&path)
                .with_context(|| format!("Unable to process manifest: {}", path.display()))?;
            Ok((path, manifest))
        })
}

fn sanitise_input(mut input: String) -> String {
    let indices: Vec<_> = input.rmatch_indices('-').map(|(i, _)| i).collect();
    for index in indices {
        input.replace_range(index..(index + 1), "_")
    }
    input
}

fn extract_with_config(process_packages: bool) -> anyhow::Result<models::Workspace> {
    let cwd = std::env::current_dir().context("Unable to locate current working directory")?;

    let mut manifests = manifests(&cwd);

    let (workspace_manifest_path, workspace_manifest) = loop {
        let Some(manifest) = manifests.next() else {
            anyhow::bail!(
                "could not find `{}` in `{}` or any parent directory",
                CARGO_TOML,
                cwd.display()
            );
        };
        match manifest {
            Ok(pair) if pair.1.workspace.is_some() => {
                break pair
            },
            Ok((_path, _)) => {
                // println!("Skipping: {} [not a workspace manifest]", path.display());
                continue;
            }
            Err(err) => {
                anyhow::bail!("Unable to read manifest: {}", err);
            }
        };
    };
    let mut root = workspace_manifest_path.clone();
    root.pop();

    // println!("Found workspace manifest: {}", root.display());
    let workspace_def = workspace_manifest
        .workspace
        .as_ref()
        .expect("Workspace should be defined, as we checked it earlier.");

    let packages = if process_packages {
        self::process_packages(&workspace_manifest, workspace_def, &root)?
    } else {
        vec![]
    };

    let metadata = workspace_def
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.get("migrate"))
        .map(|metadata| metadata.clone().try_into::<models::WorkspaceMetadata>())
        .transpose()
        .context("Unable to parse workspace metadata")?;

    let migration_sources = if let Some(metadata) = metadata.as_ref() {
        metadata
            .migrations
            .iter()
            .map(|migration| {
                let ExplicitMigration { source, schema } = migration;

                let migration = root.join(source);
                let source = migration
                    .exists()
                    .then_some(migration.clone())
                    .with_context(|| {
                        format!("Unable to locate migration source: {}", migration.display())
                    })?;

                let schema = sanitise_input(schema.clone());
                let schema = schema
                    .parse()
                    .with_context(|| format!("Invalid schema name: {}", schema))?;

                anyhow::Ok(MigrationSource {
                    source,
                    schema: Some(schema),
                })
            })
            .collect::<anyhow::Result<Vec<_>, _>>()?
    } else {
        vec![]
    };

    Ok(models::Workspace {
        root,
        metadata,
        packages,
        migration_sources,
    })
}

fn process_packages(
    workspace_manifest: &cargo_toml::Manifest,
    workspace_def: &cargo_toml::Workspace,
    root: &Path,
) -> anyhow::Result<Vec<Package>> {
    let mut packages = workspace_def
        .members
        .iter()
        .map(|member| {
            resolve_member(root, workspace_manifest, member)
                .with_context(|| format!("Unable to resolve workspace member: {}", member))
        })
        .collect::<anyhow::Result<Vec<_>, _>>()?;

    // Keep packages that have explicit migrations.
    // (We print out a helpful pointer if a package has a migrations folder, but no explicit declaration.)

    packages.retain_mut(|package| {
        let migrations = package.crate_root.join("./migrations");
        let migrations = migrations.exists().then_some(migrations);

        let opt = package
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.migrations.as_ref());

        match (opt, migrations) {
            (Some(_), _) => true,
            (None, Some(_)) => {
                println!(
                    "'{}' has a `./migrations` folder, but isn't declared in the metadata",
                    package.name
                );
                false
            }
            _ => false,
        }
    });

    let mut set = topo_sort::TopoSort::new();

    for package in packages.iter() {
        let Some(package_metadata) = package.metadata.as_ref() else {
            continue;
        };
        let Some(migrations) = package_metadata.migrations.as_ref() else {
            continue;
        };

        let mut sources = vec![];
        resolve_sources(migrations, |source, _| {
            sources.push(source.clone());
            Ok(())
        })?;

        set.insert(package.name.clone(), sources);
    }

    let topo_sort::SortResults::Full(sorted) = set.to_owned_vec_nodes() else {
        // If this happens enough, we can put in the effort to extract the non-cycle graph,
        //  and remove those nodes from the package list.
        // Leaving us with a list of packages that were involved in the cycle.
        anyhow::bail!("Cycle detected");
    };

    // This serves two functions:
    // 1) Construct a final vec of all the sorted packages. (The actual item this time being the package itself)
    // 2) Removes packages that don't have any migrations
    let mut packages = sorted
        .into_iter()
        .filter_map(|item| {
            let position = packages.iter().position(|package| package.name == item)?;
            Some(packages.swap_remove(position))
        })
        .collect::<Vec<_>>();

    super::utils::ex_iter_mut(&mut packages, |before, package, _| {
        let Some(package_metadata) = package.metadata.as_ref() else {
            // This is technically an error as the package should have been removed earlier.
            return anyhow::Ok(());
        };
        let Some(migrations) = package_metadata.migrations.as_ref() else {
            // This is technically an error as the package should have been removed earlier.
            return anyhow::Ok(());
        };

        resolve_sources(migrations, |source, schema| {
            let schema = schema
                .map(|schema| {
                    let schema = sanitise_input(schema.clone());
                    let schema = schema
                        .parse()
                        .with_context(|| format!("Invalid schema name: {}", schema))?;
                    anyhow::Ok(schema)
                })
                .transpose()?;

            let opt = before.iter().find(|package| package.name == **source);

            let Some(opt) = opt else {
                let input = package.crate_root.join(source);
                let input = input.exists().then_some(input.clone())
                    .with_context(|| {
                        format!("Unable to resolve source: {} [No crate was found with that name, and {} doesn't exist]", source, input.display())
                    })?;

                package.migration_sources.push(MigrationSource {
                    source: input,
                    schema,
                });
                return anyhow::Ok(());
            };

            package
                .migration_sources
                .extend(opt.migration_sources.clone());

            anyhow::Ok(())
        })?;

        Ok(())
    })?;

    packages.sort_unstable_by(|left, right| left.name.cmp(&right.name));

    // Skip packages that either have no migrations or have opted-out.
    packages.retain(|package| {
        !package.migration_sources.is_empty()
            && !package
                .metadata
                .as_ref()
                .map(|metadata| metadata.skip)
                .unwrap_or_default()
    });

    Ok(packages)
}

fn resolve_member<T>(
    root: &Path,
    workspace_manifest: &cargo_toml::Manifest<T>,
    member: &str,
) -> anyhow::Result<Package> {
    let path = {
        let mut member_path = root.to_path_buf();
        member_path.push(member);
        member_path.push(CARGO_TOML);
        member_path.exists().then_some(member_path)
    }
    .with_context(|| format!("Unable to find member: {}", member))?;

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Unable to read file: {}", path.display()))?;

    let mut manifest = cargo_toml::Manifest::from_str(&content)
        .with_context(|| format!("Unable to process manifest: {}", path.display()))?;

    manifest
        .complete_from_path_and_workspace(&path, Some((workspace_manifest, root)))
        .with_context(|| format!("Unable to process manifest: {}", path.display()))?;

    let package = manifest.package.as_ref().with_context(|| {
        format!(
            "Package manifest doesn't have a [package] declaration: {}",
            path.display()
        )
    })?;

    let package_metadata = package
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.get("migrate"))
        .map(|metadata| metadata.clone().try_into::<models::PackageMetadata>())
        .transpose()
        .with_context(|| format!("Unable to parse package metadata: {}", path.display()))?;

    let schema: ServiceName = {
        let name = package_metadata
            .as_ref()
            .and_then(|metadata| metadata.schema.clone())
            .unwrap_or_else(|| package.name.clone());

        let schema = sanitise_input(name);

        schema
            .parse()
            .with_context(|| format!("Invalid schema name: {}", schema))?
    };

    let crate_root = path
        .parent()
        .expect("It should have a parent")
        .to_path_buf();

    Ok(Package {
        name: package.name.clone(),
        schema_name: schema,

        crate_root,
        manifest_path: path,

        // We fill this in after collecting all the packages
        migration_sources: vec![],

        metadata: package_metadata,
    })
}

fn resolve_sources(
    migrations: &Migrations,
    mut resolve: impl FnMut(&Source, Option<String>) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    match migrations {
        Migrations::Flag(false) => (),
        Migrations::Flag(true) => {
            anyhow::bail!("migration set to true [Enabling migrations does nothing without a list of migrations]");
        }
        Migrations::Source(source) => resolve(source, None)?,
        Migrations::List(migrations) => {
            for migration in migrations.iter() {
                match migration {
                    Migration::Full(migration) => {
                        let ExplicitMigration { source, schema } = migration;
                        let schema = sanitise_input(schema.clone());
                        let schema = schema
                            .parse()
                            .with_context(|| format!("Invalid schema name: {}", schema))?;

                        resolve(source, Some(schema))?;
                    }
                    Migration::Source(source) => resolve(source, None)?,
                }
            }
        }
    }

    Ok(())
}
