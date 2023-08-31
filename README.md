## `⌽` Arranger
Arranger is a command-line utility designed to streamline development workflows in multiple programming languages.

<br>

```
Tip: when inputting a parameter that contains spaces for a command,
always wrap it around quotation marks.

# Example
arranger search -F "some name"
```

___
### `➢` Status
```
Project in development.
Currently only tested on Windows.
```

___
### `➢` Features
#### `⤷` Python Tools

| Feature                                    | Description                                                         |
|--------------------------------------------|---------------------------------------------------------------------|
| [**python venv**](#python-venv)            | Set up a new virtual environment                                    |
| [**python fix-venv**](#python-fix-venv)    | Find and resolve path issues in virtual environments                |
| [**python execute**](#python-execute)      | Find and execute commands to virtual environments                   |
| [**python packages**](#python-packages)    | Find and list packages within virtual environments                  |
| [**python download**](#python-download)    | Fetch Python versions from the official FTP server                  |

#### `⤷` Rust Tools

| Feature                                       | Description                                      |
|-----------------------------------------------|--------------------------------------------------|
| [**rust vscode-tasks**](#rust-vscode-tasks)   | Generate VSCode task configurations              |

#### `⤷` Search Tool

| Feature                 | Description                                  |
|-------------------------|----------------------------------------------|
| [**search**](#search)   | Search files on system with regex support    |

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

  - <a name="python-execute"></a>**python execute**
    ```
    Options:
    -D/--deep-search : Perform a deep search
    -C/--command : Pass command to each virtual environment

    Example:
    # Search for environments and execute "pip install numpy"
    arranger python execute -C "-m pip install numpy"
    ```

  - <a name="python-packages"></a>**python packages**
    ```
    Options:
    -D/--deep-search : Perform a deep search
    -S/--save : Save package list for each environment [$ENV/packages.txt]
    -X/--distill : Distill packages by mutual dependencies [With -S: $ENV/distilled_packages.txt]

    [$ENV placeholder refers to the root path of a Python Virtual Environment]

    Examples: 
    # Search for environments, and list the packages installed
    Example: arranger python packages

    # Search for environments, list packages, and save
    Example: arranger python packages -S

    # Search for environments, distill packages, list packages, and save
    Example: arranger python packages -S -X
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


#### `⤷` **Search Tool**
  - <a name="search"></a>**search**
    ```
    Options:
    -F/--filename : Specify Filename [Matches by start of name when used without regex]
    -E/--extensions : Specify Extensions [Can be used multiple times to add items]
    -X/--exclude-dir : Specify Directory To Exclude [Can be used multiple times to add items]
    -S/--sort : Specify Sorting Of Results [size_asc, size_desc, created_asc, created_desc, modified_asc, modified_desc]
    -L/--limit : Specify Limit For Results
    -R/--regex : Enable the regex engine for pattern matching

    Examples:
    # Search for file by name
    arranger search -F some_file

    # Search for file by name with specific extension
    arranger search -F some_file -E zip

    # Search for any file with multiple extensions 
    # (-F "") or (-F ".*") combined with -R, enables regex that captures all files
    arranger search -F "" -R -E zip -E rar -E tar

    # Search for file with regex
    arranger search -F .*some$ -R

    # Search for file with excluded directories
    arranger search -F some_file -X some_directory -X other_directory/another_directory

    # Search for file by name with results sorted by ascending size
    arranger search -F some_file -S size_asc

    # Search for file by name with results limited to 50
    arranger search -F some_file -L 50
    ```

___
### `➢` Example V0.5.14
![arranger-rs-example](https://github.com/syn-chromatic/arranger-rs/assets/68112904/939543cf-197b-4d3b-b2db-6c473855dd2c)

