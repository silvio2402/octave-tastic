# syncaud

## Requirements

- [At least one custom exception class that extends Exception should be written.](./src/client_handler.rs#L6) (The struct `BindError` is a custom exception, which defines the error type for binding errors.)
- [At least one inheritance implementation must be included.](./src/client_handler.rs#L37) (The `NetworkListener` implements the `ClientHandler` and `Listener` trait.)
- [At least one custom (non-Spring Boot interface) interface should be created and used.](./src/client_handler.rs#14) (The `ClientHandler` trait is like an interface in Rust.)

## Design Pattern

We adhered to the YAGNI (You Ain’t Gonna Need It) principle throughout the development process to avoid unnecessary complexity and over-engineering. By focusing only on features and functionality that were essential to the immediate requirements, we streamlined our implementation and kept the codebase lean. This approach not only reduced development time and effort but also ensured that the code was easier to maintain, understand, and optimize. As a result, we achieved maximum efficiency while maintaining a minimal and effective code structure tailored to meet the project’s specific needs.

## Featurelist

- The user interface should be interactive and usable by any person.
- Sounds are played synchronously across multiple devices.
- Any mp3 file can be uploaded by the user to play.

## Planned structure

![syncaud(1)](https://github.com/user-attachments/assets/264a3af8-717f-43ed-9c51-2d1da54b8c8a)

## Actual structure

![Syncaud(2)](https://github.com/user-attachments/assets/0b610dec-5589-40e5-82c3-34384649bd61)

