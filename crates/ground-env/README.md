# Ground Env

```ground-env``` is a Rust crate that provides a convenient way to load environment variables into your structs. It uses custom derive macros to automatically map environment variables to struct fields, with support for various features such as renaming fields, default values, flattening nested structures, and handling different delimiters.

## Features

- **Simple Mapping**: Map environment variables directly to struct fields.
- **Optional Fields**: Handle optional environment variables seamlessly.
- **Renaming**: Rename struct field names to match the desired environment variables.
- **Default Values**: Define default values for fields if the environment variable is missing.
- **Flattening**: Flatten nested structures.
- **Vectors**: Parse environment variables into vectors with custom delimiters.
- **Error Handling**: Graceful handling of missing required fields and invalid data types.

## Usage

Add ```ground-env``` to your ```Cargo.toml```:

```toml
[dependencies]
ground-env = "0.1.0"
```

Annotate your structs with the ```#[derive(FromEnv)]``` macro and configure the mapping using attributes:

```rust
#[derive(FromEnv)]
struct Config {
    text: String,
    optional_text: Option<String>,
    // The default delimiter is set to ","
    list: Vec<String>,
    #[env(delimiter = " ")] // But you can customise it.
    names: Vec<String>,
    number: i64,
    // Supports renaming.
    #[env(rename = "EMAIL_ADDRESS")]
    email: String,
    #[env(default)] // Defaults to 0 when no explicit value is provided.
    count: i64,
    #[env(default = "64")] // Defaults to 64 when not provided.
    background_tasks: i64,
    #[env(flatten)] // You can flatten other structs 
    admin_credentials: Credentials,
    #[env(flatten = "DB_")] // You can also provide a prefix.
    db_args: Credentials,
}

#[derive(FromEnv)]
struct Credentials {
    username: String,
    password: String,
}

fn main() -> anyhow::Result<()> {
    let t = Config::from_env()?;
    Ok(())
}
```

## License

This project is licensed under the MIT License.
See the [LICENSE](../../LICENSE) file for details.

