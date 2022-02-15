# Bash Dispatch

Bash as a language is horrible.

The weird syntax reeks of historical hacks on top of each other. Unfortunately,
until a better shell language is designed we are stuck with it. PowerShell does
a good job using objects but most configuration files and utilities still use
plain text. And plain text is eternal.

On the plus side, it's everywhere and it's easy to get started. As time goes by,
you add more and more cruft.

I'm particularly tired of using `case` to select the correct subcommand to run
from the first argument. Something like this:

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

As you add more, the case list becomes longer to maintain and it gets hard to
add nested commands. Worse, some nested commands accept value (not just
`--args`). So for a command like:

    please list repo

one would need to check not only `$1` but `$2`. Or use a 2nd case inside the
`cmd.list` function.


## Enter dispatch

Lately, I'm using a `dispatch` function that tries to find the nested command
from available functions. Like this:

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

It will try to find from declared functions with the prefix `cmd.` one that
matches the arguments you pass. It starts with all the arguments and removes
one by one until it finds a function that can be run.

Poor man's dispatch or lazy solution? Both. Always both.

It "supports" nested command by taking all the arguments and replacing any
spaces with a dot `.` and see if there's a function available.

For example, if your main script is called `please`, `please help me now` will
start by loooking for a function called `cmd.help.me.now`. If that doesn't
exist, it will look for one called `cmd.help.me`. That one also doesn't exist,
so it tries next `cmd.help` which is declare. Then it will send `me` as the `$1`
argument to it.

In case `cmd.help` doesn't exist, it calls the function name inside the
`DEFAULT` variable.

The existence of a `PREFIX` means other functions will not be searched in the
lookup process.


## One more thing

I gave up showing a script's usage using `echo`.

It's much cleaner to write the usage in a comment in the beginning of the file
and grep it. It's also the first thing you see when you open the script in a
text editor.

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


## Todos

- [ ] Ignore everything after the first flag
- [ ] Think on the ambiguity between a valid value and a command. For
      example: is `repo` in `please list repo` a value to `cmd.list` or the
      command `cmd.list.repo`? This is what keeps me awake at night.
