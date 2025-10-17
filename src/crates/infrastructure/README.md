# Infrastructure Crate

This crate provides the infrastructure layer for the application, according to the Domain-Driven Design (DDD) architecture. It contains concrete implementations of the interfaces defined in the `domain` crate.

## Modules

The crate is organized into modules based on the type of infrastructure:

-   `mysql`: Provides implementations that interact with a MySQL database.
-   `in_memory_db`: Provides in-memory implementations, primarily for testing and development purposes.

## Implementations

This crate provides implementations for the following domain interfaces:

-   `CircleRepository`: For persisting and retrieving circle aggregates.
    -   MySQL implementation: `infrastructure::mysql::circle_repository::CircleRepository`
    -   In-memory implementation: `infrastructure::in_memory_db::circle_repository::CircleRepository`
-   `CircleDuplicateChecker`: For checking if a circle with the same name already exists.
    -   MySQL implementation: `infrastructure::mysql::circle_duplicate_checker::CircleDuplicateChecker`
    -   In-memory implementation: `infrastructure::in_memory_db::circle_duplicate_checker::CircleDuplicateChecker`

## Database Schema

The `db_schema` module defines the data structures that map to the database tables.


## Testing
The crate includes unit tests for each implementation, ensuring that they conform to the expected behavior defined in the domain interfaces. Tests are located in the respective module directories.

```bash
cargo test --package infrastructure
```