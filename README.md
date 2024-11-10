# Rust-Based Terminal File Manager

When Ranger stopped working on Fedora, I decided to take matters into my own hands and build a simple, customizable terminal-based file manager in Rust. This project covers the basics of directory navigation, file previews, and metadata display, offering a lightweight alternative to traditional file managers.

---

## Features

This file manager is minimal yet functional, aiming to simplify terminal file navigation without extra dependencies.

- **Navigation**: Use the arrow keys and Enter to browse directories.
  - **Up/Down Arrow Keys**: Scroll through files and folders in the current directory.
  - **Right Arrow**: Enter a folder.
  - **Left Arrow**: Go up one level in the directory.
  - **`q`**: Quit the application.

---

## Room for Improvement

While this project covers basic functionality, there are many features to be added:

- **Asynchronous File Operations**: Improve speed for larger directories.
- **Image Previews**: Add support for displaying ASCII or other text-based previews for images.
- **Search**: Enable filtering of files on the fly.
- **File Preview Enhancements**: Expand file preview capabilities to include more file types.

This project is far from a full-featured Ranger clone, but it does the essentials and has room for expansion.

---

## Getting Started

### Prerequisites

Make sure you have Rust installed on your system to run this program. As this is still in development, I haven’t created a standalone executable yet. You’ll need to run it directly with `cargo`.

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/your-repo-name.git
   cd your-repo-name
   ```

2. **Run the Project**:

   ```bash
   cargo run
   ```

3. **Navigation**: Use the arrow keys and `Enter` to navigate as described above.

---

## Contributing

Feel free to fork this repository and contribute! Submit a pull request or open an issue if you have suggestions for improvements.

---

### Note

This file manager was born out of necessity, but it has proven to be a rewarding experience in exploring Rust and terminal-based applications. It’s not perfect, but it works for my needs—and maybe it will for yours too!

---
