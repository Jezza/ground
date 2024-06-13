// use anyhow::Context as _;
// use env::FromEnvironment;
// use spaces::{EnvironmentKind, SpaceKind, SpaceOwner};
use std::collections::HashMap;
use std::hash::Hash;
// use std::{ops::Deref, str::FromStr};
//
// use sqlx::{query, Executor, Postgres};
//
// pub use sqlx;
// pub use sqlx::postgres::PgPool;
//
// use commons::cli::postgres::PostgresArgs;
//
// use super::{extraction, migrate_prelude, ServiceName};

// pub async fn make_database(
//     schema: Option<ServiceName>,
//     package_name: Option<String>,
// ) -> (PgPool, ServiceName) {
//     try_make_database(schema, package_name)
//         .await
//         .expect("we can never fail")
// }

// async fn try_make_database(
//     schema: Option<ServiceName>,
//     package_name: Option<String>,
// ) -> anyhow::Result<(PgPool, ServiceName)> {
//     let mut pg = postgres_args()?;
//     pg.min_connections = 0;
//     let pool = pg.connect().await.context("failed to connect to db")?;
//     let database_name = ServiceName::random();
//
//     crate::create_database(&pool, &database_name)
//         .await
//         .context("Unable to create database")?;
//
//     pg.database = database_name.deref().into();
//
//     tracing::info!("Creating extensions");
//
//     let mut workspace = if package_name.is_some() {
//         extraction::extract()?
//     } else {
//         extraction::extract_workspace()?
//     };
//
//     let pool = pg.connect().await?;
//     let mut db = pool.acquire().await?;
//
//     let packages = std::mem::take(&mut workspace.packages);
//
//     migrate_prelude(
//         workspace,
//         &mut db,
//         true,
//         false,
//         false,
//         // EnvironmentKind::Test,
//         // SpaceKind::Partner,
//         // SpaceOwner::from_str("testingdock").expect("This cannot fail"),
//     )
//     .await?;
//
//     let schema = if let Some(package_name) = package_name {
//         if let Some(package) = packages
//             .into_iter()
//             .find(|package| package.name == package_name)
//         {
//             let schema = schema.clone().unwrap_or(package.schema_name);
//
//             crate::migrate_sources(
//                 &mut db,
//                 &package.migration_sources,
//                 Some(schema.clone()),
//                 true,
//                 true,
//                 false,
//                 true,
//                 false,
//             )
//             .await?;
//
//             Some(schema)
//         } else if let Some(schema) = schema {
//             crate::prepare_schema(&mut *db, &schema, false, true).await?;
//             Some(schema)
//         } else {
//             None
//         }
//     } else if let Some(schema) = schema {
//         crate::prepare_schema(&mut *db, &schema, false, true).await?;
//         Some(schema)
//     } else {
//         None
//     };
//
//     let pool = match schema {
//         Some(schema) => pg.connect_with_schema(&schema.as_db_name()).await,
//         None => pg.connect().await,
//     }
//     .context("failed to connect to db")?;
//
//     Ok((pool, database_name))
// }

// pub async fn drop_database(name: ServiceName) {
//     try_drop_database(name).await.expect("we will never fail")
// }
//
// pub async fn bootstrap_database<'a, E>(executor: E) -> anyhow::Result<()>
// where
//     E: Executor<'a, Database = Postgres>,
// {
//     query("select bootstrap.bootstrap_v6()")
//         .execute(executor)
//         .await?;
//     Ok(())
// }
//
// async fn try_drop_database(name: ServiceName) -> anyhow::Result<()> {
//     let mut pg = postgres_args()?;
//     pg.min_connections = 0;
//     let pool = pg.connect().await.context("failed to connect to db")?;
//
//     pool.execute(&*format!(
//         r#"DROP DATABASE {} WITH (FORCE);"#,
//         name.as_db_name()
//     ))
//     .await
//     .context("Failed to drop database")?;
//
//     Ok(())
// }
//
// fn postgres_args() -> Result<PostgresArgs, env::Error> {
//     PostgresArgs::from_environment(None)
// }

/// Exclusive iterator.
///
/// Loops over a slice, while giving you access to the other elements in the slice.
pub(crate) fn ex_iter_mut<T, F, E>(slice: &mut [T], mut func: F) -> Result<(), E>
where
    F: FnMut(&mut [T], &mut T, &mut [T]) -> Result<(), E>,
{
    for index in 0..slice.len() {
        let (before, rest) = slice.split_at_mut(index);
        let (item, after) = rest.split_first_mut().expect("We're checking the length");
        func(before, item, after)?;
    }
    Ok(())
}

pub fn group_by<T, GK: Eq + Hash, GV>(
    items: impl IntoIterator<Item = T>,
    group: impl Fn(T) -> (GK, GV),
) -> HashMap<GK, Vec<GV>> {
    let mut groups: HashMap<GK, Vec<GV>> = HashMap::new();
    for item in items {
        let (key, value) = group(item);
        groups.entry(key).or_default().push(value);
    }
    groups
}

pub fn ordered_group_by<T, GK: PartialEq + Eq + Hash, GV>(
    items: impl IntoIterator<Item = T>,
    group: impl Fn(T) -> (GK, GV),
) -> Vec<(GK, Vec<GV>)> {
    let mut groups: Vec<(GK, Vec<GV>)> = vec![];
    for item in items {
        let (key, value) = group(item);
        let entry = groups.iter_mut().find(|(item, _)| key == *item);
        match entry {
            Some((_, items)) => {
                items.push(value);
            }
            None => {
                groups.push((key, vec![value]));
            }
        }
    }

    groups
}
