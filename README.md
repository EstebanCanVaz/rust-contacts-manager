# Rust Contacts Manager

A command-line contact management system built in Rust that allows you to store, search, and manage contacts with their birthdays.

## Features

- **Add Contacts**: Store contact information including name, birthday, phone, and email
- **Search Contacts**: Find contacts by name (case-insensitive)
- **Birthday Filtering**: List all contacts with birthdays in a specific month
- **Alphabetical Listing**: View all contacts sorted alphabetically
- **Delete Contacts**: Remove contacts by name
- **Data Persistence**: Automatically saves contacts to JSON file
- **Date Validation**: Validates birthday dates according to month rules

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/YOUR_USERNAME/rust-contacts-manager.git
cd rust-contacts-manager
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

## Usage

The application provides an interactive menu with the following options:

1. **Add Contact**: Enter name, birthday (day/month), phone, and email
2. **Search Contact**: Find a contact by name
3. **List by Month**: Show all contacts with birthdays in a specific month
4. **List All**: Display all contacts sorted alphabetically
5. **Delete Contact**: Remove a contact by name
6. **Exit**: Save and quit the application

## Data Storage

Contacts are automatically saved to `contactos.json` in the project directory. The file uses a readable JSON format:

```json
[
  {
    "nombre": "John Doe",
    "cumple": "15/03",
    "telefono": "123456789",
    "correo": "john@example.com"
  }
]
```

## Project Structure

```
src/
├── main.rs          # Main application logic
contactos.json       # Contact data storage
Cargo.toml          # Project dependencies
.gitignore          # Git ignore rules
README.md           # This file
```

## Dependencies

- `serde`: Serialization/deserialization
- `serde_json`: JSON file handling

## Development

To run in development mode:
```bash
cargo run
```

To build for release:
```bash
cargo build --release
```

## License

This project is open source and available under the MIT License.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Author

Created as part of a Rust learning project. 