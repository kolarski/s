# S - Architecture Documentation

## Project Overview

`s` is a user-friendly command-line wrapper around the Linux `screen` utility. It provides a simpler interface to the powerful but complex `screen` command, making it more accessible to developers who don't want to remember all the cryptic flags and key combinations.

## Architectural Style

This project implements **Clean Architecture** with principles from **Domain-Driven Design (DDD)**. This architectural approach provides several benefits:

1. **Separation of concerns** - Each layer has a specific responsibility
2. **Dependency rule** - Dependencies only point inward
3. **Testability** - Business logic can be tested independently of external systems
4. **Flexibility** - Implementation details can be changed without affecting the core business logic

## Directory Structure

```
src/
├── domain/               # Core business entities and logic
│   └── entities/         # Domain objects
├── application/          # Application-specific business rules
│   ├── ports/            # Interfaces (ports) to external systems
│   └── use_cases/        # Application-specific business logic
├── infrastructure/       # External adapters and implementations
│   ├── adapters/         # Adapters for external tools
│   └── repositories/     # Implementations of repository interfaces
├── presentation/         # UI layer
│   └── formatters/       # Output formatting
├── lib.rs                # Public API exports
└── main.rs               # Entry point and dependency wiring
```

## Layer Details

### Domain Layer

The Domain layer contains the core business entities and rules that are independent of any external system.

#### Key Components:

- **ScreenSession Entity** (`domain/entities/screen_session.rs`)

  - Core domain entity representing a screen session
  - Has properties like id, name, created_at, and status
  - Implements domain-specific behavior related to screen sessions

- **SessionStatus Enum** (`domain/entities/screen_session.rs`)
  - Value object representing the possible states of a screen session (Attached, Detached, Unknown)

This layer is isolated from external concerns and does not depend on any other layer.

### Application Layer

The Application layer contains use cases and ports (interfaces) that define how the application interacts with the outside world.

#### Key Components:

- **ScreenRepository Interface** (`application/ports/screen_repository.rs`)

  - Defines methods for managing screen sessions (list, create, attach, detach, kill)
  - Acts as a port/interface that must be implemented by the infrastructure layer

- **ListScreenSessions Use Case** (`application/use_cases/list_screen_sessions.rs`)
  - Defines the business logic for listing screen sessions
  - Depends on the ScreenRepository interface, not its implementation
  - Single Responsibility: List screen sessions

### Infrastructure Layer

The Infrastructure layer contains concrete implementations of interfaces defined in the Application layer and adapters for external tools and libraries.

#### Key Components:

- **ScreenCommand Adapter** (`infrastructure/adapters/screen_command.rs`)

  - Adapter for interacting with the Linux `screen` command
  - Encapsulates the details of command execution

- **ScreenRepositoryImpl** (`infrastructure/repositories/screen_repository.rs`)
  - Concrete implementation of the ScreenRepository interface
  - Uses the ScreenCommand adapter to interact with the screen utility
  - Parses command output into domain entities

### Presentation Layer

The Presentation layer is responsible for formatting and displaying data to the user.

#### Key Components:

- **TableFormatter** (`presentation/formatters/table_formatter.rs`)
  - Formats lists of ScreenSession entities into nice tabular output
  - Separates display concerns from business logic

### Main Entry Point

The `main.rs` file acts as a composition root, wiring together all components:

- Creates infrastructure components (ScreenCommand, ScreenRepositoryImpl)
- Creates use cases (ListScreenSessions)
- Coordinates the flow of data between layers
- Handles top-level error display

## Data Flow

1. **User Input** → main.rs
2. main.rs creates and configures components
3. main.rs delegates to appropriate use case
4. Use case interacts with repository through interface
5. Repository uses adapter to execute actual screen commands
6. Repository parses raw output into domain entities
7. Use case returns domain entities
8. main.rs passes entities to formatter
9. Formatter creates user-friendly output
10. **Output** → User

## Dependency Direction

Following Clean Architecture principles, dependencies always point inward:

- Domain layer depends on nothing
- Application layer depends only on Domain layer
- Infrastructure layer depends on Application and Domain layers
- Presentation layer depends on Domain layer
- Main depends on all layers

## Key Design Patterns

1. **Repository Pattern** - Abstracts data access behind interfaces
2. **Adapter Pattern** - Adapts external systems to work with our application
3. **Dependency Injection** - Components receive their dependencies rather than creating them
4. **Use Case Pattern** - Encapsulates business logic in single-responsibility classes

## Development Guidelines

### Adding New Features

1. **Start with the domain** - Define entities and business rules
2. **Define interfaces** - Create or update interfaces in the application layer
3. **Implement use cases** - Add business logic in application layer
4. **Implement adapters/repositories** - Add concrete implementations in infrastructure layer
5. **Update presentation** - Format and display results
6. **Wire in main.rs** - Connect new components

### Example: Adding Screen Window Management

1. Add WindowSession entity to domain layer (if needed)
2. Add window management methods to ScreenRepository interface
3. Create CreateWindow use case in application layer
4. Implement window management in ScreenRepositoryImpl
5. Add window-specific formatting in presentation layer
6. Wire the new use case in main.rs

## Error Handling

The application uses Rust's Result type for error handling:

- Infrastructure failures return specific errors
- Each layer can add context to errors
- The main.rs handles top-level error display

## Future Improvements

1. **Command-line argument parsing** - Add support for command-line arguments
2. **Configuration** - Add support for user configuration
3. **More screen features** - Implement more screen functionality
4. **Testing** - Add unit and integration tests
5. **Better error messages** - Improve error messages and handling
6. **Documentation** - Add more documentation

## Technologies Used

- **Rust** - Systems programming language
- **Standard Library** - For process execution and file I/O
- **Screen** - Linux utility for terminal session management

## Conclusion

This architecture provides a solid foundation for maintainable, testable code with clear separation of concerns. By following these patterns, developers can extend the application while maintaining its architectural integrity.
