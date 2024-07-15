//! Implements a postgres specific clap struct.
//! This can be flattened within clap commands, or used directly.
//! It provides a simple standard that can be adhered to throughout the different crates.

use std::path::PathBuf;
use std::str::FromStr;

use clap::ValueEnum;
use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgPool};

pub const DB_URI_SHORT_HELP: &str = r###"
When you use this option, all defaults set by this application will be ignored.

The general form for a connection URI is:
    postgresql://[user[:password]@][host][:port][/dbname][?param1=value1&...]
"###;

pub const DB_URI_LONG_HELP: &str = r###"
When you use this option, all defaults set by this application will be ignored.

The general form for a connection URI is:
    postgresql://[user[:password]@][host][:port][/dbname][?param1=value1&...]

┌─────────────────────────────┬─────────────────────┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Parameter                   │ Default             │ Description                                                                                                     │
├─────────────────────────────┼─────────────────────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ `sslmode`                   │ `prefer`            │ Determines whether or with what priority a secure SSL TCP/IP connection will be negotiated. See [`PgSslMode`].  │
│ `sslrootcert`               │ `None`              │ Sets the name of a file containing a list of trusted SSL Certificate Authorities.                               │
│ `statement-cache-capacity`  │ `100`               │ The maximum number of prepared statements stored in the cache. Set to `0` to disable.                           │
│ `host`                      │ `None`              │ Path to the directory containing a PostgreSQL unix domain socket, which will be used instead of TCP if set.     │
│ `hostaddr`                  │ `None`              │ Same as `host`, but only accepts IP addresses.                                                                  │
│ `application-name`          │ `None`              │ The name will be displayed in the pg_stat_activity view and included in CSV log entries.                        │
│ `user`                      │ result of `whoami`  │ PostgreSQL user name to connect as.                                                                             │
│ `password`                  │ `None`              │ Password to be used if the server demands password authentication.                                              │
│ `port`                      │ `5432`              │ Port number to connect to at the server host, or socket file name extension for Unix-domain connections.        │
│ `dbname`                    │ `None`              │ The database name.                                                                                              │
│ `options`                   │ `None`              │ The runtime parameters to send to the server at connection start.                                               │
└─────────────────────────────┴─────────────────────┴─────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

The URI scheme designator can be either `postgresql://` or `postgres://`.
Each of the URI parts is optional.

    postgresql://
    postgresql://localhost
    postgresql://localhost:5433
    postgresql://localhost/mydb
    postgresql://user@localhost
    postgresql://user:secret@localhost
    postgresql://localhost?dbname=mydb&user=postgres&password=postgres
"###;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "env", derive(ground_env::FromEnv))]
#[cfg_attr(feature = "env", clap(next_help_heading = "POSTGRES", term_width = 200))]
pub struct PostgresArgs {
    /// The general form for a connection URI is:
    ///
    /// ```text
    /// postgresql://[user[:password]@][host][:port][/dbname][?param1=value1&...]
    /// ```
    ///
    /// ## Parameters
    ///
    /// |Parameter|Default|Description|
    /// |---------|-------|-----------|
    /// | `sslmode` | `prefer` | Determines whether or with what priority a secure SSL TCP/IP connection will be negotiated. See [`PgSslMode`]. |
    /// | `sslrootcert` | `None` | Sets the name of a file containing a list of trusted SSL Certificate Authorities. |
    /// | `statement-cache-capacity` | `100` | The maximum number of prepared statements stored in the cache. Set to `0` to disable. |
    /// | `host` | `None` | Path to the directory containing a PostgreSQL unix domain socket, which will be used instead of TCP if set. |
    /// | `hostaddr` | `None` | Same as `host`, but only accepts IP addresses. |
    /// | `application-name` | `None` | The name will be displayed in the pg_stat_activity view and included in CSV log entries. |
    /// | `user` | result of `whoami` | PostgreSQL user name to connect as. |
    /// | `password` | `None` | Password to be used if the server demands password authentication. |
    /// | `port` | `5432` | Port number to connect to at the server host, or socket file name extension for Unix-domain connections. |
    /// | `dbname` | `None` | The database name. |
    /// | `options` | `None` | The runtime parameters to send to the server at connection start. |
    ///
    /// The URI scheme designator can be either `postgresql://` or `postgres://`.
    /// Each of the URI parts is optional.
    ///
    /// ```text
    /// postgresql://
    /// postgresql://localhost
    /// postgresql://localhost:5433
    /// postgresql://localhost/mydb
    /// postgresql://user@localhost
    /// postgresql://user:secret@localhost
    /// postgresql://localhost?dbname=mydb&user=postgres&password=postgres
    /// ```
    ///
    #[cfg_attr(feature = "clap", clap(name = "postgres-uri", env = "POSTGRES_URI", long, display_order = 0, help = DB_URI_SHORT_HELP, long_help = DB_URI_LONG_HELP))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_URI"))]
    pub uri: Option<String>,

    /// Sets the name of the host to connect to.
    ///
    /// If a host name begins with a slash, it specifies Unix-domain communication
    /// rather than TCP/IP communication; the value is the name of the directory
    /// in which the socket file is stored.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-host",
        env = "POSTGRES_HOST",
        default_value = "localhost",
        long,
        conflicts_with = "postgres-uri"
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_HOST"))]
    pub host: String,

    /// Sets the username to connect as.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-user",
        env = "POSTGRES_USER",
        default_value = "postgres",
        long,
        conflicts_with = "postgres-uri"
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_USER", default = "postgres"))]
    pub user: String,

    /// Sets the password to use if the server demands password authentication.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-password",
        env = "POSTGRES_PASSWORD",
        default_value = "password",
        long,
        conflicts_with = "postgres-uri"
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_PASSWORD", default = "password"))]
    pub password: SecretString,

    /// Sets the port to connect to at the server host.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-port",
        env = "POSTGRES_PORT",
        default_value = "5432",
        long,
        conflicts_with = "postgres-uri"
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_PORT", default = "5432"))]
    pub port: u16,

    /// Sets the database name
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-database",
        env = "POSTGRES_DATABASE",
        default_value = "postgres",
        long,
        conflicts_with = "postgres-uri"
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_DATABASE", default = "postgres"))]
    pub database: String,

    /// Sets the schema name. In postgres terms, this is the search path.
    /// Does not conflict with postgres-uri.
    #[cfg_attr(feature = "clap", clap(name = "postgres-schema", env = "POSTGRES_SCHEMA", long))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_SCHEMA"))]
    pub schema: Option<String>,

    /// Sets whether or with what priority a secure SSL TCP/IP connection will be negotiated
    /// with the server.
    ///
    /// By default, the SSL mode is [`Prefer`](PgSslMode::Prefer), and the client will
    /// first attempt an SSL connection but fallback to a non-SSL connection on failure.
    ///
    /// Ignored for Unix domain socket communication.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-ssl-mode",
        env = "POSTGRES_SSL_MODE",
        default_value = "prefer",
        long,
        display_order = 2000,
        conflicts_with = "postgres-uri",
        value_enum
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_SSL_MODE", default = "prefer"))]
    pub ssl_mode: PgSslMode,

    /// Sets the name of a file containing SSL certificate authority (CA) certificate(s).
    /// If the file exists, the server's certificate will be verified to be signed by
    /// one of these authorities.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-ssl-root-cert",
        env = "POSTGRES_SSL_ROOT_CERT",
        display_order = 2001,
        long,
        conflicts_with = "postgres-uri",
        value_hint = clap::ValueHint::FilePath,
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_SSL_ROOT_CERT"))]
    pub ssl_root_cert: Option<PathBuf>,

    /// Enables logging of statements.
    ///
    /// Can be useful for debugging queries.
    #[cfg_attr(feature = "clap", clap(
        name = "postgres-enable-statement-logging",
        env = "POSTGRES_ENABLE_STATEMENT_LOGGING",
        display_order = 2002,
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_ENABLE_STATEMENT_LOGGING", default))]
    pub enable_statement_logging: bool,

    /// If true, the health of a connection will be verified by a call to Connection::ping
    /// before returning the connection.
    #[cfg_attr(feature = "clap", clap(
        name = "test-before-acquire",
        env = "POSTGRES_TEST_BEFORE_ACQUIRE",
        default_value = "false",
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_TEST_BEFORE_ACQUIRE", default))]
    pub test_before_acquire: bool,

    /// Set the maximum number of connections that this pool should maintain.
    ///
    /// Be mindful of the connection limits for your database as well as other applications
    /// which may want to connect to the same database (or even multiple instances of the
    /// same application in high-availability deployments).
    #[cfg_attr(feature = "clap", clap(
        name = "max-connections",
        env = "POSTGRES_MAX_CONNECTIONS",
        default_value = "10",
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_MAX_CONNECTIONS", default = "10"))]
    pub max_connections: u32,

    /// Set the maximum amount of time to spend waiting for a connection in Pool::acquire().
    ///
    /// Caps the total amount of time Pool::acquire() can spend waiting across multiple phases:
    /// - First, it may need to wait for a permit from the semaphore, which grants it the
    ///   privilege of opening a connection or popping one from the idle queue.
    /// - If an existing idle connection is acquired, by default it will be checked for liveness
    ///   and integrity before being returned, which may require executing a command on the
    ///   connection. This can be disabled with test_before_acquire(false).
    ///   - If before_acquire is set, that will also be executed.
    /// - If a new connection needs to be opened, that will obviously require I/O, handshaking,
    ///   and initialization commands.
    ///   - If after_connect is set, that will also be executed.
    #[cfg_attr(feature = "clap", clap(
        name = "acquire-timeout-seconds",
        env = "POSTGRES_ACQURIE_TIMEOUT_SECONDS",
        default_value = "30",
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_ACQURIE_TIMEOUT_SECONDS", default = "30"))]
    pub acquire_timeout: u64,

    /// Set the minimum number of connections to maintain at all times.
    ///
    /// When the pool is built, this many connections will be automatically spun up.
    ///
    /// If any connection is reaped by max_lifetime or idle_timeout, or explicitly closed, and
    /// it brings the connection count below this amount, a new connection will be opened to replace it.
    ///
    /// This is only done on a best-effort basis, however. The routine that maintains this value has
    /// a deadline so it doesn’t wait forever if the database is being slow or returning errors.
    ///
    /// This value is clamped internally to not exceed max_connections.
    ///
    /// We’ve chosen not to assert min_connections <= max_connections anywhere because it shouldn’t
    /// break anything internally if the condition doesn’t hold, and if the application allows
    /// either value to be dynamically set then it should be checking this condition itself and
    /// returning a nicer error than a panic anyway.
    #[cfg_attr(feature = "clap", clap(
        name = "min-connections",
        env = "POSTGRES_MIN_CONNECTIONS",
        default_value_t = 1,
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_MIN_CONNECTIONS", default = "1"))]
    pub min_connections: u32,

    /// Set the maximum lifetime of individual connections.
    ///
    /// Any connection with a lifetime greater than this will be closed.
    ///
    /// When set to None, all connections live until either reaped by idle_timeout or explicitly disconnected.
    ///
    /// Infinite connections are not recommended due to the unfortunate reality of memory/resource leaks
    /// on the database-side. It is better to retire connections periodically (even if only once daily)
    /// to allow the database the opportunity to clean up data structures (parse trees, query metadata
    /// caches, thread-local storage, etc.) that are associated with a session.
    #[cfg_attr(feature = "clap", clap(
        name = "max-lifetime-seconds",
        env = "POSTGRES_MAX_LIFETIME_SECONDS",
        default_value = "1800",
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_MAX_LIFETIME_SECONDS", default = "1800"))]
    pub max_lifetime: u64,

    /// Set a maximum idle duration for individual connections.
    ///
    /// Any connection that remains in the idle queue longer than this will be closed.
    ///
    /// For usage-based database server billing, this can be a cost saver.
    #[cfg_attr(feature = "clap", clap(
        name = "idle-timeout-seconds",
        env = "POSTGRES_IDLE_TIMEOUT_SECONDS",
        default_value = "600",
        long
    ))]
    #[cfg_attr(feature = "env", env(rename = "POSTGRES_IDLE_TIMEOUT_SECONDS", default = "600"))]
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PgSslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl From<PgSslMode> for sqlx::postgres::PgSslMode {
    fn from(value: PgSslMode) -> Self {
        match value {
            PgSslMode::Disable => sqlx::postgres::PgSslMode::Disable,
            PgSslMode::Allow => sqlx::postgres::PgSslMode::Allow,
            PgSslMode::Prefer => sqlx::postgres::PgSslMode::Prefer,
            PgSslMode::Require => sqlx::postgres::PgSslMode::Require,
            PgSslMode::VerifyCa => sqlx::postgres::PgSslMode::VerifyCa,
            PgSslMode::VerifyFull => sqlx::postgres::PgSslMode::VerifyFull,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Unknown PgSslMode variant: {0}")]
pub struct UnknownPgSslMode(String);

impl FromStr for PgSslMode {
    type Err = UnknownPgSslMode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "disabled" => Ok(Self::Disable),
            "allow" => Ok(Self::Allow),
            "prefer" => Ok(Self::Prefer),
            "require" => Ok(Self::Require),
            "verifyca" => Ok(Self::VerifyCa),
            "verifyfull" => Ok(Self::VerifyFull),
            _ => Err(UnknownPgSslMode(s.into())),
        }
    }
}

static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

impl PostgresArgs {
    pub async fn connect(&self) -> sqlx::Result<PgPool> {
        self.impl_connect_with_database_and_schema(&self.database, &self.schema)
            .await
    }

    /// Connect, but override the schema.
    pub async fn connect_with_schema(&self, schema: &str) -> sqlx::Result<PgPool> {
        self.impl_connect_with_database_and_schema(&self.database, &Some(schema.into()))
            .await
    }

    /// Connect, but override the database.
    pub async fn connect_with_database(&self, database: &str) -> sqlx::Result<PgPool> {
        self.impl_connect_with_database_and_schema(database, &self.schema)
            .await
    }

    /// Connect, but override the database and schema.
    pub async fn connect_with_database_and_schema(
        &self,
        database: &str,
        schema: &str,
    ) -> sqlx::Result<PgPool> {
        self.impl_connect_with_database_and_schema(database, &Some(schema.into()))
            .await
    }

    pub fn build_options(&self) -> sqlx::Result<PgConnectOptions> {
        self.build_options_with_database_and_schema(&self.database, &self.schema)
    }

    pub fn build_pool_options(&self) -> sqlx::pool::PoolOptions<sqlx::Postgres> {
        sqlx::pool::PoolOptions::default()
            .test_before_acquire(self.test_before_acquire)
            .max_connections(self.max_connections)
            .acquire_timeout(core::time::Duration::from_secs(self.acquire_timeout))
            .min_connections(self.min_connections)
            .max_lifetime(Some(core::time::Duration::from_secs(self.max_lifetime)))
            .idle_timeout(Some(core::time::Duration::from_secs(self.idle_timeout)))
            .after_connect(|conn, meta| {
                Box::pin(async move {
                    let sql = "show transaction_read_only";
                    let transaction_read_only: String =
                        sqlx::query_scalar(sql).fetch_one(conn).await?;
                    if transaction_read_only == "on" {
                        tracing::error!(metadata = ?meta, "pg connection is read-only");
                        Err(sqlx::Error::Configuration(
                            "Cannot run in read-only transaction".into(),
                        ))
                    } else {
                        tracing::debug!(metadata = ?meta, "pg pool established a new connection");
                        Ok(())
                    }
                })
            })
            .before_acquire(|conn, meta| {
                Box::pin(async move {
                    let sql = "show transaction_read_only";
                    let transaction_read_only: String =
                        sqlx::query_scalar(sql).fetch_one(conn).await?;
                    if transaction_read_only == "on" {
                        tracing::error!(metadata = ?meta, "pg connection is read-only");
                        Ok(false)
                    } else {
                        tracing::trace!(metadata = ?meta, "aquiring connection from pg pool");
                        Ok(true)
                    }
                })
            })
            .after_release(|_conn, meta| {
                Box::pin(async move {
                    tracing::trace!(metadata = ?meta, "connection was released back to pg pool");
                    Ok(true)
                })
            })
    }

    fn build_options_with_database_and_schema(
        &self,
        database: &str,
        schema: &Option<String>,
    ) -> sqlx::Result<PgConnectOptions> {
        let PostgresArgs {
            uri,
            host,
            port,
            user,
            password,
            database: _, // This is intentionally ignored.
            schema: _,   // This is intentionally ignored.
            ssl_mode,
            ssl_root_cert,
            enable_statement_logging,
            test_before_acquire: _,
            max_connections: _,
            acquire_timeout: _,
            min_connections: _,
            max_lifetime: _,
            idle_timeout: _,
        } = self;

        // @FIXME jezza - 30 Aug 2022: Don't get me started on this shit.
        //  PgConnectOptions contains a field that can be used to define extra connection options.
        //  This includes things like the search_path, with that we can define the "defined" schema
        //  for a given connection.
        //  Apparently, in some infinite wisdom well beyond the scope of mere mortals, the guys working
        //  on sqlx have decided that this field should be gated by a completely different feature, and
        //  as a result, leads to incompatibilities, as we want the form "--key=value", but the
        //  thing they exposed was a stupid key-value mapping that's mapped down to a "-c key=value", which is subtly
        //  different.
        //  What's even stupider is that this value, during construction of the struct, is pulled out of the env.
        //  You can probably see where I'm going with this.
        //  We want to populate this field, and do so correctly.
        //  So as a stop-gap until sqlx provide a "better" accessor, I lock the region, and replace the ENV variable with
        //  the thing that we want to slot into place.
        //  The issue, of course, rises from when infra wants to define this value.
        //  We should definitely add support to just append to the existing value, but at this point, I'm so annoyed with
        //  sqlx that I'll leave it for the next poor soul to wander into this region of code.

        const PG_OPTIONS: &str = "PGOPTIONS";

        let _guard = LOCK
            .lock()
            .expect("If we can't get the lock, we are screwed");

        // let ssl_mode = ssl_mode.unwrap_or_default();
        let old_value = std::env::var_os(PG_OPTIONS);

        if let Some(schema) = schema {
            let value = format!("--search_path={},public", schema);
            std::env::set_var(PG_OPTIONS, value);
        };

        let opts = if let Some(uri) = uri {
            PgConnectOptions::from_str(uri)?.database(database)
        } else {
            // @TODO jezza - 31 May 2022: Any other options?
            let opts = PgConnectOptions::new()
                .host(host)
                .port(*port)
                .username(user)
                .password(password.expose_secret())
                .database(database)
                .ssl_mode((*ssl_mode).into());
            let opts = if !enable_statement_logging {
                opts.disable_statement_logging()
            } else {
                opts
            };

            if let Some(ssl_root_cert) = ssl_root_cert {
                opts.ssl_root_cert(ssl_root_cert)
            } else {
                opts
            }
        };

        if let Some(value) = old_value {
            std::env::set_var(PG_OPTIONS, value);
        } else {
            std::env::remove_var(PG_OPTIONS);
        }

        Ok(opts)
    }

    /// Connects to a Postgres instance with the specified database and schema.
    async fn impl_connect_with_database_and_schema(
        &self,
        database: &str,
        schema: &Option<String>,
    ) -> sqlx::Result<PgPool> {
        let pool_opts = self.build_pool_options();
        let opts = self.build_options_with_database_and_schema(database, schema)?;
        let pool = pool_opts.connect_with(opts).await?;
        Ok(pool)
    }
}
