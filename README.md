## Arranger
Arranger is a command-line utility designed to streamline development workflows in multiple programming languages.

`Project in development.`
___
### `➢` Features
#### `⤷` Python Tools

| Feature                                    | Description                                                         |
|--------------------------------------------|---------------------------------------------------------------------|
| [**python venv**](#python-venv)            | Set up a new virtual environment                                    |
| [**python fix-venv**](#python-fix-venv)    | Find and resolve path issues in virtual environments                |
| [**python packages**](#python-packages)    | Find and list packages within virtual environments                  |
| [**python download**](#python-download)    | Fetch specific Python versions from the official FTP server         |

#### `⤷` Rust Tools

| Feature                                       | Description                                      |
|-----------------------------------------------|--------------------------------------------------|
| [**rust vscode-tasks**](#rust-vscode-tasks)   | Generate VSCode task configurations              |

___
### `➢` **Usage**
#### `⤷` **Python Tools**
  - <a name="python-venv"></a>**python venv**
    ```
    Options:
    -V/--version : Specify Python version

    Example:
    # Create Virtual Environemnt for Python 3.9
    arranger python venv -V 3.9
    ```

  - <a name="python-fix-venv"></a>**python fix-venv**
    ```
    Options:
    -D/--deep-search : Perform a deep search

    Example: 
    # Search for environments and fix path issues
    arranger python fix-venv
    ```

  - <a name="python-packages"></a>**python packages**
    ```
    Options:
    -D/--deep-search : Perform a deep search
    -S/--save-packages : Creates a packages.txt file in each environment

    Examples: 
    # Search for environments, and list the packages installed
    Example: arranger python packages

    # Search for environments, list the packages, and save
    Example: arranger python packages -S
    ```

  - <a name="python-download"></a>**python download**
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
  - <a name="rust-vscode-tasks"></a>**rust vscode-tasks**
    ```
    Options:
    -R/--run-task : Generate VSCode run task configuration

    Example: arranger rust vscode-tasks -R
    ```

___
### `➢` Example
![arranger](https://github.com/syn-chromatic/arranger-rs/assets/68112904/e581e0f7-2921-475b-a123-f52251bdbd65)
