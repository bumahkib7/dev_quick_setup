# DevQuickSetup

DevQuickSetup is a powerful and user-friendly CLI tool designed to streamline the setup of development environments on macOS. Whether you're using an Intel-based Mac or an Apple M1 chip, DevQuickSetup ensures a smooth and efficient configuration process.

## Features

- Easy installation and setup of common development tools and languages.
- Customizable configurations for basic, full, or personalized development setups.
- Cross-compatibility with both Intel and Apple Silicon (M1) architectures.
- User-friendly interface with interactive prompts.

## Installation

To install DevQuickSetup, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/bumahkib7/DevQuickSetup.git
    ```

2. Navigate to the cloned directory:

    ```bash
    cd DevQuickSetup
    ```

3. Build the project using Cargo:

    ```bash
    cargo build --release
    ```

4. Initialize the application (this will set up `devsetup` as a global command):

    ```bash
    sudo ./target/release/dev_quick_setup init
    ```

## Usage

After installation, you can use the `devsetup` command in your terminal. Here are some of the common commands:

- To start the basic setup:

    ```bash
    devsetup basic
    ```

- To start the full setup:

    ```bash
    devsetup full
    ```

- To customize your setup:

    ```bash
    devsetup customized
    ```

## Contributing

Contributions to DevQuickSetup are welcome! Please feel free to submit pull requests, report bugs, or suggest new features.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- Thanks to all the contributors who have helped with the development of DevQuickSetup.
- Special thanks to the Rust community for their invaluable resources and support.
