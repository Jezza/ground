use super::*;
use futures_util::future::BoxFuture;
use sqlx::error::BoxDynError;
use sqlx::migrate;
use sqlx::migrate::Migration;

#[allow(clippy::too_many_arguments)]
pub async fn migrate_sources(
    db: &mut PgConnection,
    sources: &[models::MigrationSource],
    default_schema: Option<ServiceName>,
    include_schema: bool,
    include_migrations: bool,
    destroy_existing_schema: bool,
    create_missing_schema: bool,
    ignore_missing: bool,
) -> anyhow::Result<()> {
    let groups = utils::ordered_group_by(sources.iter(), |source: &models::MigrationSource| {
        let models::MigrationSource { source, schema } = source;

        let schema = schema
            .as_ref()
            .or(default_schema.as_ref())
            .expect("No default schema specified");

        (schema, source.clone())
    });

    for (schema, sources) in groups {
        info!("Applying migrations for {}", schema);

        let source = SourceCollection { sources };

        migrate(
            &mut *db,
            source,
            schema,
            include_schema,
            include_migrations,
            destroy_existing_schema,
            create_missing_schema,
            ignore_missing,
        )
        .await?;
    }

    Ok(())
}

#[derive(Debug)]
struct SourceCollection<T> {
    sources: Vec<T>,
}

impl<'s, T> migrate::MigrationSource<'s> for SourceCollection<T>
where
    T: migrate::MigrationSource<'s> + Send + 's,
{
    fn resolve(self) -> BoxFuture<'s, Result<Vec<Migration>, BoxDynError>> {
        Box::pin(async move {
            let mut migrations = vec![];
            for source in self.sources {
                info!("Resolving: {:?}", source);
                let migration = source.resolve().await?;
                migrations.extend(migration);
            }
            info!("Applying migrations");
            Ok(migrations)
        })
    }
}

#[tracing::instrument(skip_all, fields(%schema))]
#[allow(clippy::too_many_arguments)]
async fn migrate(
    db: &mut PgConnection,
    source: impl migrate::MigrationSource<'_>,
    schema: &ServiceName,
    // @FIXME jeremy.barrow - 13 June 2024: Replace all of these flags with policies or something...
    include_schema: bool,
    include_migrations: bool,
    destroy_existing_schema: bool,
    create_missing_schema: bool,
    ignore_missing: bool,
) -> anyhow::Result<()> {
    if include_schema {
        info!("Preparing schema");

        prepare_schema(
            &mut *db,
            schema,
            destroy_existing_schema,
            create_missing_schema,
        )
        .await?;
    }

    if include_migrations {
        info!("Setting search path");
        set_search_path(&mut *db, schema).await?;

        migrate_source(&mut *db, source, ignore_missing).await?;
    }

    Ok(())
}

async fn migrate_source(
    executor: impl PgExecutor<'_> + Acquire<'_, Database = Postgres>,
    source: impl migrate::MigrationSource<'_>,
    ignore_missing: bool,
) -> sqlx::Result<()> {
    info!("Resolving migrations");

    Migrator::new(source)
        .await?
        .set_ignore_missing(ignore_missing)
        .run(executor)
        .await?;

    info!("Finished migration");

    Ok(())
}
