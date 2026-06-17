# Clickhouse Migration Tool (chm) - README

## Overview

The Clickhouse Migration Tool (`chm`) is a command-line interface (CLI) designed to help manage database migrations for Clickhouse. It generates new migrations and runs them against ClickHouse in a stateful manner. Connection details are read from environment variables.

## Installation

To install the Clickhouse Migration Tool, you need to have Rust installed on your machine. If you don't already have it installed, you can install it with:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install it from the fork:

```sh
cargo install --git https://github.com/sergafon/clickhouse-migrations
```

After installing the tool, you can run the CLI using the `chm` command.

## Connection

Connection details are read from environment variables (a `.env` file in the working directory is loaded automatically):

- `CLICKHOUSE_URL` (required) — ClickHouse HTTP URL, e.g. `http://127.0.0.1:8123`.
- `CLICKHOUSE_DB` (required) — target database.
- `CLICKHOUSE_USER` (optional) — applied if set.
- `CLICKHOUSE_PASSWORD` (optional) — applied if set.

## Migrations directory

Migrations live in the directory given by `-s`/`--source` (default `ch_migrations/`), resolved relative to the current working directory. The flag is global and applies to every `migration` subcommand.

## Usage

### General Structure

```sh
chm [--source <DIR>] <command> [subcommand] [flags]
```

### Global flags

- `-s, --source <DIR>` — directory containing migrations (default `ch_migrations`). Global; applies to all `migration` subcommands.

### Commands and Subcommands

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

1. **Generate a New Migration**

   ```sh
   chm migration generate create_users_table
   ```

2. **Run Pending Migrations**

   ```sh
   chm migration run
   ```

3. **Redo the Latest Migration**

   ```sh
   chm migration redo
   ```

4. **Revert the Last Migration**

   ```sh
   chm migration revert
   ```

5. **Run Migrations from a Custom Directory**

   ```sh
   chm migration run --source migrations/clickhouse
   ```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contact

For any questions or issues, please open an issue on the GitHub repository or contact the maintainers.

---

This README provides a basic overview of the Clickhouse Migration Tool and its features. For detailed usage and examples, please refer to the command-specific help by running `chm <command> --help`.
