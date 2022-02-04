# Bash Dispatch

Bash as a language is horrible.

The weird syntax reeks of historical hacks on top of each other. Unfortunately,
until a better shell language is designed we are stuck with it. PowerShell does
a good job using objects but most configuration files and utilities still use
plain text. And plain text is eternal.

On the plus side, it's everywhere and it's easy to get started. As time goes by,
you add more and more crufty code.

I'm particularly tired of using `case` to select the correct subcommand to run
from a script. Something like this:

```bash
#!/usr/bin/env bash
set -euo pipefail

cmd.list() {
  ls -alF $HOME
}

cmd.help() {
  echo "Usage: something <command>"
  echo
  echo "Commands:"
  echo "    list          # List all files in the home directory"
}

cmd.other() {
  echo "==> Args: $@ ($#)"
}

COMMAND="${1:-list}
shift

case "$COMMAND" in
  "list")     cmd.list $@ ;;
  "help")     cmd.help $@ ;;
  "other")    cmd.other $@ ;;
  #...
esac
```

The `case` selects the correct subcommand to run. As you add more, it becomes a
long case list.

Lately, I'm using a `dispatch` functions that tries to find the subcommand from
available functions. Like this:

```bash
dispatch() {
  : "Finding command for $@..."
  local PREFIX="cmd"
  local max=${#@}

  if [ $max -lt 1 ]
  then
    cmd.help
    return
  fi

  while true
  do
    arg=${@:1:$max}
    cmd="${arg// /.}"
    : "Checking $PREFIX.$cmd: $(type $PREFIX.$cmd 2>/dev/null | head -n 1)"
    if type $PREFIX.$cmd >/dev/null 2>&1
    then
      args=${@:$(($max+1))}
      : "Calling $PREFIX.$cmd($args)"
      eval $PREFIX.$cmd $args
      return
    fi

    max=$(( max - 1))
    if [ $max -lt 1 ]
    then
      >&2 echo "Error: Couldn't find command for '$@'. Try '$0 help'."
      return 1
    fi
  done

  echo >&2 "Error: Command '$1' not found."
  return 1
}

dispatch $@

```

This function will try to find from functions declared with the prefix `cmd.`
one that matches the arguments you pass.

It "supports" nested command by taking all the arguments and replacing any
spaces with a dot `.` and see if there's a function available.

For example, if your main script is called `please`, `please help me` will start
by loooking for a function called `cmd.help.me`. If that doesn't exist, it will
look for one called `cmd.help`. Since that one does exists, it will send `me` as
the `$1` argument to it.

If `cmd.help` doesn't exist, it calls the `DEFAULT` function.

The existence of a `PREFIX`, means other functions will not be used in the
lookup process.


### One more thing

I also get tired to update the usage echo strings.

It's much easier to write the usage in a comment in the beginning of the file
and grep it. Example:

```bash
#!/usr/bin/env bash
### USAGE
###     please <command>
###
### COMMANDS
###    list          # List all files in the home directory"
###    other         # Does something else"
###    help [WHAT]   # It supposedly helps with WHAT"
set -euo pipefail

cmd.help() {
  grep '^###' $0 | cut -c 5-
}

cmd.help
```

### The whole shebang

```bash
#!/usr/bin/env bash
### USAGE
###     please <command>
###
### COMMANDS
###    list          # List all files in the home directory"
###    other         # Does something else"
###    help [WHAT]   # It supposedly helps with WHAT"
set -euo pipefail

cmd.help() {
  grep '^###' $0 | cut -c 5-
}

cmd.list() {
  ls -alF --color=auto $HOME
}

cmd.other() {
  echo "==> Args: $@ ($#)"
}

dispatch() {
  : "Finding command for $@..."
  local PREFIX="cmd"
  local DEFAULT="cmd.help"
  local max=${#@}

  if [ $max -lt 1 ]
  then
    $DEFAULT
    return
  fi

  while true
  do
    arg=${@:1:$max}
    cmd="${arg// /.}"
    : "Checking $PREFIX.$cmd: $(type $PREFIX.$cmd 2>/dev/null | head -n 1)"
    if type $PREFIX.$cmd >/dev/null 2>&1
    then
      args=${@:$(($max+1))}
      : "Calling $PREFIX.$cmd($args)"
      eval $PREFIX.$cmd $args
      return
    fi

    max=$(( max - 1))
    if [ $max -lt 1 ]
    then
      >&2 echo "Error: Couldn't find command for '$@'. Try '$0 help'."
      return 1
    fi
  done

  echo >&2 "Error: Command '$1' not found."
  return 1
}

dispatch $@
```

### Todos

- [ ] Ignore everything after flags
