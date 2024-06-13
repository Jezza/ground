use anyhow::Context;
use futures_util::TryStreamExt;
use sqlx::{migrate::Migrator, query, Acquire, Executor, PgConnection, PgExecutor, Postgres};
use tracing::info;

pub use types::*;

pub use migration::migrate_sources;

mod migration;
pub mod models;

pub mod utils;

pub mod extraction;

mod types;

/// Operates on the provided schema within the same transaction.
#[tracing::instrument(skip_all, fields(%schema))]
pub async fn prepare_schema<'a, E>(
    pool: E,
    schema: &ServiceName,
    destroy_existing_schema: bool,
    create_missing_schema: bool,
) -> sqlx::Result<()>
where
    E: PgExecutor<'a> + Acquire<'a, Database = Postgres>,
{
    if destroy_existing_schema || create_missing_schema {
        let mut trans = pool.begin().await?;

        if destroy_existing_schema {
            destroy_schema(&mut *trans, schema).await?;
        }

        if create_missing_schema {
            create_schema(&mut *trans, schema).await?;
        }

        trans.commit().await?;
    }

    Ok(())
}

pub async fn set_search_path(db: &mut PgConnection, schema: &str) -> anyhow::Result<()> {
    query(&format!("set search_path = {}, public;", schema))
        .execute(&mut *db)
        .await
        .context(format!("Failed to switch to {} schema.", schema))?;

    Ok(())
}

/// Creates a new database if it doesn't already exist.
pub async fn create_database<'a, E>(executor: E, database: &ServiceName) -> sqlx::Result<()>
where
    E: PgExecutor<'a>,
{
    info!(name=%database, "Creating database");

    let sql = format!("CREATE DATABASE {};", database.as_db_name());
    query(&sql).execute(executor).await?;

    Ok(())
}

/// False vacuum decay option. Obliterates a database, no take backs.
pub async fn destroy_database<'a, E>(
    executor: E,
    database: &ServiceName,
    force: bool,
) -> sqlx::Result<()>
where
    E: PgExecutor<'a>,
{
    info!(name=%database, "Destroying database");

    let sql = if force {
        format!(
            "DROP DATABASE IF EXISTS {} WITH (FORCE);",
            database.as_db_name()
        )
    } else {
        format!("DROP DATABASE IF EXISTS {};", database.as_db_name())
    };
    query(&sql).execute(executor).await?;

    Ok(())
}

/// Nuclear option. Destroys a schema, no take backs.
pub async fn destroy_schema<'a, E>(executor: E, service_name: &ServiceName) -> sqlx::Result<()>
where
    E: PgExecutor<'a>,
{
    info!(name=%service_name, "Destroying schema");

    // `ServiceName` names are always valid SQL identifiers.
    let sql = format!(
        "drop schema if exists {} cascade;",
        service_name.as_db_name()
    );
    query(&sql).execute(executor).await?;

    Ok(())
}

/// Creates a new schema if it doesn't already exist.
pub async fn create_schema<'a, E>(executor: E, service_name: &ServiceName) -> sqlx::Result<()>
where
    E: PgExecutor<'a>,
{
    info!(name=%service_name, "Creating schema");

    let sql = format!("create schema if not exists {};", service_name.as_db_name());
    query(&sql).execute(executor).await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn migrate_prelude(
    workspace: models::Workspace,
    db: &mut PgConnection,
    create_missing_schema: bool,
    destroy_existing_schema: bool,
    ignore_missing: bool,
) -> anyhow::Result<()> {
    let sources = workspace.migration_sources;
    if sources.is_empty() {
        return Ok(());
    }

    info!("Migrating directories included via cargo metadata");

    let mut tx = db.begin().await.context("Unable to start transaction")?;

    migrate_sources(
        &mut tx,
        &sources,
        None,
        true,
        true,
        destroy_existing_schema,
        create_missing_schema,
        ignore_missing,
    )
    .await?;

    tx.commit().await?;

    info!("Finished migrating workspace migration sources");

    Ok(())
}
