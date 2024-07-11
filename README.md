# Clickhouse Migration Tool (chm) - README

## Overview

The Clickhouse Migration Tool (`chm`) is a command-line interface (CLI) designed to help manage database migrations for Clickhouse. It allows users to set up migration configurations, generate new migrations, and run migrations in a stateful manner.

## Installation

To install the Clickhouse Migration Tool, you need to have Rust installed on your machine. If you don't already have it installed, you can install it with:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, you can install trieve from cargo:

```sh
cargo install chm
```

After installing the tool, you can run the CLI using the `chm` command.

## Usage

### General Structure

```sh
chm <command> [subcommand] [flags]
```

### Commands and Subcommands

#### Setup

Initial setup of the migration tool. This command creates a folder to contain migrations and a `.toml` file with connection details. It will error if the migrations folder already exists.

```sh
chm setup --url <CLICKHOUSE_URL> --user <CLICKHOUSE_USER> --password <CLICKHOUSE_PASSWORD> --database <CLICKHOUSE_DB>
```

- `--url` (Optional): Clickhouse URL. Will look for `CLICKHOUSE_URL` environment variable if not provided.
- `--user` (Optional): Clickhouse user. Will look for `CLICKHOUSE_USER` environment variable if not provided.
- `--password` (Optional): Clickhouse password. Will look for `CLICKHOUSE_PASSWORD` environment variable if not provided.
- `--database` (Optional): Clickhouse database. Will look for `CLICKHOUSE_DB` environment variable if not provided.

#### Migration

Commands to handle migration operations.

##### Generate

Generates a new migration file with the specified name.

```sh
chm migration generate <MIGRATION_NAME>
```

##### Run

Identifies and runs pending migrations.

```sh
chm migration run
```

##### Redo

Reverts the latest migration and applies it again.

```sh
chm migration redo
```

##### Revert

Reverts the last migration.

```sh
chm migration revert
```

## Example

1. **Setup the Migration Tool**

   ```sh
   chm setup --url http://localhost:8123 --user default --password password --database my_database
   ```

2. **Generate a New Migration**

   ```sh
   chm migration generate create_users_table
   ```

3. **Run Pending Migrations**

   ```sh
   chm migration run
   ```

4. **Redo the Latest Migration**

   ```sh
   chm migration redo
   ```

5. **Revert the Last Migration**

   ```sh
   chm migration revert
   ```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

For any questions or issues, please open an issue on the GitHub repository or contact the maintainers.

---

This README provides a basic overview of the Clickhouse Migration Tool and its features. For detailed usage and examples, please refer to the command-specific help by running `chm <command> --help`.
