# Shell written in Rust

A shell written in Rust which supports following commands:

- echo
- type
- exit

## echo command

Syntax: **echo** \<string_to_print>

prints the argument string to the terminal.

###### Example:

```
$ echo hello world
> hello world
```

## type command

Syntax: **type** \<command>

Type command has two uses:

#### Tell about builtin commands

###### Example:

```
$ type type
> type is shell builtin
```

#### Search for the command in PATH

If the command specified in the argument is not a builtin command, it searches for the executable file in the directories specified by PATH environment variable, and if a file is found, it prints the path to that file.

###### Example:

```
$ type ls
> ls is /usr/bin/ls
```

## exit command

Syntax: **exit** \<exit_code>

Exit the shell.
