# Shell Interpreter

This project is a simple shell interpreter written in Rust. It uses the `ratatui` library for creating a text-based user interface and `clap` for command-line argument parsing.

## How to Run

To run this project, you need to have Rust and Cargo installed. You can run the project using the following command:

```sh
cargo run
```

## Using Ratatui

This project uses the `ratatui` library to create a text-based user interface. `ratatui` is a Rust library for building rich terminal user interfaces and dashboards.

## Using Clap

The `clap` library is used for parsing command-line arguments. It provides a simple way to define and parse commands and their arguments.

## Commands

The shell interpreter supports the following commands:

### `cd`

Changes the current directory to the specified path.

**Usage:**

```sh
cd [path]
```

**Example:**

```sh
cd /home/user/projects
```

### `ls`

Lists the files and directories in the current directory or the specified path.

**Usage:**

```sh
ls [options] [dir] -> [output]
```

**Options:**

- `-a, --all`: List all files including hidden files.
- `-l, --long`: List all files in long format.

**Example:**

```sh
ls -a -l /home/user/projects -> output.txt
```

### `mkdir`

Creates a new directory.

**Usage:**

```sh
mkdir [options] [directories...]
```

**Options:**

- `-p, --parents`: Create parent directories if they do not exist.
- `-v, --verbose`: Print information about the directories created.

**Example:**

```sh
mkdir -p -v new_directory
```

### `touch`

Creates a new file.

**Usage:**

```sh
touch [options] [files...]
```

**Options:**

- `-v, --verbose`: Print information about the files created.

**Example:**

```sh
touch -v new_file.txt
```

### `mv`

Moves or renames files and directories.

**Usage:**

```sh
mv [options] [sources...] [target]
```

**Options:**

- `-f, --force`: Overwrite existing files.
- `-v, --verbose`: Print information about the directories/files moved.

**Example:**

```sh
mv -f -v file1.txt file2.txt /new_directory
```

### `cp`

Copies files and directories.

**Usage:**

```sh
cp [options] [sources...] [target]
```

**Options:**

- `-r, --recursive`: Copy directories recursively.
- `-f, --force`: Overwrite existing files.
- `-v, --verbose`: Print information about the directories/files copied.

**Example:**

```sh
cp -r -f -v dir1 dir2 /new_directory
```

### `rm`

Removes files and directories.

**Usage:**

```sh
rm [options] [targets...]
```

**Options:**

- `-r, --recursive`: Remove directories recursively.
- `-f, --force`: Ignore nonexistent files and missing operands without error.
- `-v, --verbose`: Print information about the directories/files removed.

**Example:**

```sh
rm -r -f -v old_directory
```

### `cat`

Displays the contents of a file.

**Usage:**

```sh
cat [options] [files...] -> [output]
```

**Options:**

- `-n, --number`: Display line numbers.
- `-b, --blanknon`: Number non-blank output lines.
- `-s, --squeeze`: Suppress repeated empty output lines.

**Example:**

```sh
cat -n file1.txt file2.txt -> output.txt
```

### `clear`

Clears the terminal screen.

**Usage:**

```sh
clear
```

### `Esc`

Exits the shell interpreter.


## Examples

Here are some examples of how to use the shell interpreter:

```sh
# Change directory
cd /home/user/projects

# List files in the current directory
ls

# List files in a specific directory
ls -a -l /home/user/projects -> output.txt

# Create a new directory
mkdir -pv new_directory

# Create a new file
touch -v new_file.txt

# Move a file to a new directory
mv -f -v file1.txt /new_directory

# Copy a directory to a new location
cp -rfv dir1 /new_directory

# Remove a directory
rm -r -f -v old_directory

# Display the contents of a file
cat -n file1.txt -> output.txt

# Clear the terminal screen
clear
```


