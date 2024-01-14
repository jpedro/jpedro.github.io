<!-- tags: shell, markdown -->

# Bash dispatch

<!-- START TAGS -->
[<img src="https://img.shields.io/badge/Tag-shell-brightgreen">](/tags/shell)
[<img src="https://img.shields.io/badge/Tag-markdown-brightgreen">](/tags/markdown)
<!-- END TAGS-->

Bash as a language is horrible.

The weird syntax reeks of historical hacks on top of each other.
Unfortunately, until a better shell is wide-spread we are stuck with it.
PowerShell does a good job using objects.

On the plus side, it's everywhere and it's easy to get started. You just
throw all shell commands into a file. As time goes by, you always add
more cruft, like conditionals, loops and functions.


## Subcommands

I grew particularly tired of using `case` to select the correct
subcommand / function to run from the first argument. Something like
this:

```bash
#!/usr/bin/env bash
set -euo pipefail

cmd.list() {
    ls -alF ${1:-$HOME}
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

As you add more functionality, the case list becomes longer to maintain
and it gets hard to add nested commands. Some nested commands might
accept values that can also be `--flag`s. So for a command like:

    please list --repos

one would need to check not only `$1` but `$2`. Or use a 2nd case inside
the `cmd.list` function. And `--repos` is a valid direcotry name.


## Enter dispatch

Lately, I'm using a `dispatch` function that tries to find the
subcommand from the declared functions, with a known prefix:

```bash

# All of the above...

dispatch() {
    : "Finding command for $@..."
    local prefix="cmd"
    local fallback="cmd.help"
    local max=${#@}

    if [ $max -lt 1 ]
    then
        $fallback
        return
    fi

    while true
    do
        arg=${@:1:$max}
        cmd="${arg// /.}"
        : "Checking $prefix.$cmd: $(type $prefix.$cmd 2>/dev/null | head -n 1)"
        if type $prefix.$cmd >/dev/null 2>&1
        then
            args=${@:$(($max+1))}
            : "Calling $prefix.$cmd($args)"
            eval $prefix.$cmd $args
            return
        fi

        max=$(( max - 1))
        if [ $max -lt 1 ]
        then
            echo >&2 "Error: Couldn't find command for '$@'. Try '$0 help'."
            return 1
        fi
    done

    echo >&2 "Error: Command '$1' not found."
    return 1
}

dispatch $@

```

It will try to find `cmd.$1` from declared functions. The longest one
that matches the arguments you pass. It starts with all the arguments
and removes one by one until it finds a function that can be run.

Poor man's dispatcher or lazy solution? Both. Always both!

It "supports" nested command by taking replacing any spaces with a
dot `.` and check if that function is available.

For example, if your main script is called `please`, `please help me
now` will start by loooking for a function called `cmd.help.me.now`. If
that doesn't exist, it will look for one called `cmd.help.me`. That one
also doesn't exist, so it tries next `cmd.help` which is declared. Then
it will send `me now` as the arguments to it.

In case `cmd.help` doesn't exist, it calls the function name inside the
`fallback` variable.

The existence of a `prefix` means other functions will not be searched
in the lookup process. It's a guard.


## One more thing

I gave up showing a script's usage using `echo`s. It's so ugly.

So now, I just slap the usage in a comment in the beginning of the file
and grep for it. It's also thas the benefit it's the first thing you see
when you open the script in a code editor.

```bash
#!/usr/bin/env bash
### USAGE
###     please <command>
###
### COMMANDS
###    list          # List all files in the home directory
###    other         # And now for something completely different
###    help [WHAT]   # It supposedly helps with WHAT
set -euo pipefail

cmd.help() {
    grep '^###' $0 | cut -c 5-
}

# ...
```


## The whole #!shebang

```bash
#!/usr/bin/env bash
### USAGE
###     please <command>
###
### COMMANDS
###    list          # List all files in the home directory
###    other         # And now for something completely different
###    help [WHAT]   # It supposedly helps with WHAT
set -euo pipefail

cmd.help() {
    grep '^###' $0 | cut -c 5-
}

cmd.list() {
    ls -alF --color=auto ${1:-$HOME}
}

cmd.other() {
    echo "==> Args: $@ ($#)"
}

dispatch() {
    : "Finding command for $@..."
    local prefix="cmd"
    local fallback="cmd.help"
    local max=${#@}

    if [ $max -lt 1 ]
    then
        $fallback
        return
    fi

    while true
    do
        arg=${@:1:$max}
        cmd="${arg// /.}"
        : "Checking $prefix.$cmd: $(type $prefix.$cmd 2>/dev/null | head -n 1)"
        if type $prefix.$cmd >/dev/null 2>&1
        then
            args=${@:$(($max+1))}
            : "Calling $prefix.$cmd($args)"
            eval $prefix.$cmd $args
            return
        fi

        max=$(( max - 1))
        if [ $max -lt 1 ]
        then
            echo >&2 "Error: Couldn't find command for '$@'. Try '$0 help'."
            return 1
        fi
    done

    echo >&2 "Error: Command '$1' not found."
    return 1
}

dispatch $@
```


## Todos

- Ignore everything after the first flag is found

- Store the found flags in a map

- Think on the ambiguity between a valid value and a command. For
  example: should `repo` in `please list repo` be the `$1` value
  to `cmd.list` or the command `cmd.list.repo`? This is the stuff
  that keeps me awake at night.

- Stick that `dispatch` into a snippet (oh the joy of escaping it all!)















<!-- START FOOTER -->
 &nbsp;

<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js"></script>
<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-database.js"></script>
<script src="https://jpedro.github.io/js/v1/data.js"></script>
<script src="https://jpedro.github.io/js/v1/comments.js"></script>
<script defer="">Comments.mount(document.body.children[0]);</script>
<!-- END FOOTER -->












