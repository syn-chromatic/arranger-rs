## Arranger
Arranger is a command-line utility designed to streamline development workflows in multiple programming languages.

`Project in development.`
___
### `➢` Features
#### `⤷` Python Tools

| Feature                      | Description                                                         |
|------------------------------|---------------------------------------------------------------------|
| [**venv**](#venv)            | Set up a new virtual environment                                    |
| [**fix-venv**](#fix-venv)    | Find and resolve path issues in virtual environments                |
| [**packages**](#packages)    | Find and list packages within virtual environments                  |
| [**download**](#download)    | Fetch specific Python versions from the official FTP                |

#### `⤷` Rust Tools

| Feature                             | Description                                      |
|-------------------------------------|--------------------------------------------------|
| [**vscode-tasks**](#vscode-tasks)   | Generate VSCode task configurations              |

___
### `➢` **Usage**

#### `⤷` **Python Tools**
  - [**venv**](#venv)
    ```
    Options:
    -V/--version : Specify Python version

    Example: arranger python venv -V 3.9
    ```

  - [**fix-venv**](#fix-venv)
    ```
    Options:
    -D/--deep-search : Perform a deep search

    Example: arranger python fix-venv
    ```

  - [**packages**](#packages)
    ```
    Options:
    -S/--save-packages : Creates a packages.txt file in each environment

    Example: arranger python packages -S
    ```

  - [**download**](#download)
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
  - [**vscode-tasks**](#vscode-tasks)
    ```
    Options:
    -R/--run-task : Generate VSCode run task configuration

    Example: arranger rust vscode-tasks -R
    ```

___
### `➢` Example
![arranger](https://github.com/syn-chromatic/arranger-rs/assets/68112904/e581e0f7-2921-475b-a123-f52251bdbd65)
