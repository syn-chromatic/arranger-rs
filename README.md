## Arranger
Arranger is a command-line utility designed to streamline development workflows in multiple programming languages.

`Project in development.`
___
### `➢` Features
#### `⤷` Python Tools
- **venv**: Create a virtual environment
- **fix-venv**: Find and repair virtual environment path issues
- **packages**: Find and list the packages installed in virtual environments
- **download**: Download any version of Python from their FTP server

#### `⤷` Rust Tools
- **vscode-tasks**: Generate tasks for VSCode

___
### `➢` Example
![arranger](https://github.com/syn-chromatic/arranger-rs/assets/68112904/e581e0f7-2921-475b-a123-f52251bdbd65)


___
### `➢` **Usage**

#### `⤷` **Python Tools**
  - **venv**
    ```
    Options:
    -V/--version : Specify Python version

    Example: arranger python venv -V 3.9
    ```

  - **fix-venv**
    ```
    Options:
    -D/--deep-search : Perform a deep search

    Example: arranger python fix-venv
    ```

  - **packages**
    ```
    Options:
    -S/--save-packages : Creates a packages.txt file in each environment

    Example: arranger python packages -S
    ```

  - **download**
    ```
    Options:
    -V/--version : Specify Python version
    -R/--recent-patch : Retrieve most recent patch
    -L/--list : List Python version files [No Download]
    -A/--arch : Specify Architecture [amd64, arm64, n/a] [default: amd64]
    -P/--platform : Specify Platform [windows, macos, any] [default: windows]
    -T/--package-type : Specify Package Type [standard, webinstall, embed, source] [default: standard]

    Examples:
    # Get specific Python version for Windows
    arranger python download -V 3.9.6

    # Get latest Python patch version for Windows
    arranger python download -V 3.9 -R

    # Get latest Python patch vesion for MacOS
    arranger python download -V 3.9 -P macos -A n/a -R

    # Get latest Python patch version source
    arranger python download -V 3.9 -P any -A n/a -T source -R
    ```

#### `⤷` **Rust Tools**
  - **vscode-tasks**
    ```
    Options:
    -R/--run-task : Generate Run Task

    Example: arranger rust vscode-tasks -R
    ```
