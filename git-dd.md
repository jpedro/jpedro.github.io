<!-- tags: git, shell -->

# `git-dd`

My current favourite git alias is `dd`.

What does it do? Let's ask git.

```
$ git help dd
'dd' is aliased to 'deploy'
```

O kay. That was... *helpful*... but what's `git deploy` then?

It's another alias. `dd` is a git finger-friendly alias to yet another
git alias.

> "Turtles all the way down"
>
> — Older flat eather

```
$ git help deploy
'deploy' is aliased to '!f(){ host=$(git config deploy.host); dir=$(git config deploy.dir); if [[ $host = '' ]] || [[ $dir = '' ]] ; then echo 'Git config deploy is not configured.'; return; fi; git pp; echo '\033[2m'; ssh -A $host 'cd '$dir' && git ff && git log -1'; echo '\033[0m' ;};f'
```

That's quite a bit to process. Let's break it down:

- The `!f() { ... ;};f` sets the git alias to become a shell
  function. Inside this function we:

- Grab the `deploy.host` and `deploy.dir` git config  values.

- Ensure they are not empty. We exit early with a message if empty.

- Run some `ssh` command.


## Git aliases

You read it. Git alias can host not only alias to 1. other git commands
and flags, but also to 2. shell calls and 3. inline shell functions.


### 1. Command alias

```ini
[alias]
    st = status
    me = branch --show-current
```

So now `git st` is short for `git status`. And `git me` is short for
`git branch --show-current` (or if you are using an older git version
set it to `rev-parse --abbrev-ref HEAD`).


### 2. Shell calls

```ini
[alias]
    date   = "!date +'%Y-%m-%d'"
    parent = "!git config branch.$(git me).remote" # Can't use `remote`
```

The leading `!` indicates this is an external shell call.

`git date` is just an example of calling the `/usr/date` with the
format as the first argument.

Also note how `git parent` calls `git` itself as an external program
and accepts subshell commands (the `$(command)` part). In this case,
it will use the alias we created above `git me` and will resolve the
upstream remote name for the current branch, if it exists.


### 3. Shell functions

```ini
[alias]
    yo = "!f(){ echo "Yo, ${@:-dude}! 🍪" ;};f"
```

The leading `!` tells us we are calling the shell, then the `f(){ ... }`
tells we are creating a function, and the last `f` means we are
calling it immediately. The extra semicolons are required for
one-liners.

`yo` is now an alias for that inline shell function. If you call
`git yo` it will answer.

```
$ git yo
Yo, dude! 🍪
```

If you feel bold, you pass an argument:

```
$ git yo a monad is a monoid in the category of endofunctors
Yo, a monad is a monoid in the category of endofunctors! 🍪
```

![But why](https://raw.githubusercontent.com/jpedro/jpedro.github.io/master/.github/static/img/why.jpg)

Because functions are more flexible that commands. You have `if`
conditionals and `for` loops, they accept multiple arguments, download
the internets and stream Veep. As your heart always wanted to, but you
just didn't know.


## Git config

Git config is a key value store in
[INI format](https://en.wikipedia.org/wiki/INI_file).

You can store values both in the local repo's `.git/config` or in the
global `~/.gitconfig` file. The command is unsurprisingly
`git config [ENTRY] [VALUE]`. The INI `ENTRY` is formed by a section
and a key name joined by a `.`. For example this local git repo has
this sectrion:

```ini
[branch "master"]
    remote = origin
    merge = refs/heads/master
```

Which is what we used for the `git parent` alias above. In that case,
the `ENTRY` is `branch.master.remote` and `branch.master` is the
section, `remote` being the key.

You need to quote the section name part if they are further dots in it.
For example:

```
$ grep -A 1 hi .git/config
[hi "are.you"]
    ok = "FINE, I'VE NEVER BEEN BETTER!!!"

$ git config hi.are.you.ok
FINE, I'VE NEVER BEEN BETTER!!!
```

To read an entry pass only the `ENTRY`. To set an entry pass both
`ENTRY` and `VALUE`.

And you are not limited to git's internal known values. You can add
your own.

Armed with this knowledge we can understand now how this unholy
`git deploy` contraption works.



## Git deploy

> "Finger-friendly turtles all the way down"
>
> — Wiser flat eather

If we break down the inline function in the `deploy` alias, indent it
propertly and remove extraneous semicolons, it looks like a pretty
run-of-the-mill function:

```bash
f() {
    host=$(git config deploy.host)
    dir=$(git config deploy.dir)

    if [[ $host = '' ]] || [[ $dir = '' ]]
    then
        echo 'Git config deploy is not configured.'
        return
    fi

    git pp
    echo '\033[2m'
    ssh -A $host 'cd '$dir' && git ff && git log -1'
    echo '\033[0m'
}
```

It checks if `host` and `dir` are present and then ssh's into the host
and does `git ff` in that directory.

Here's `git pp` and `git pp` along with the rest:

```ini
[alias]
    pp = "!f(){ git ss \"$1\" && [[ $(git parent) != '' ]] && git push || git push $(git primus) HEAD -u ;};f"
    ff = "!f(){ git fetch $@ && git rebase $(git upstream) || (git rebase --abort && echo '==> Failed to rebase' && exit 1);};f"
    ss = "!f(){ git add --all && git cc \"$1\" ;};f"
    cc = "!f(){ git commit --verbose -m \"${1:-$(git message)}\" || true ;};f"

    parent   = "!git config branch.$(git name).remote"  # Can't use 'remote'
    upstream = "!git rev-parse --abbrev-ref @{u} 2>/dev/null || echo '(none)'"
    primus   = "!git remote get-url origin >/dev/null 2>&1 && echo origin || git remote | head -1"
    message  = "!commitment 2>/dev/null || curl -sfL whatthecommit.com/index.txt || echo 'This reveals a lack of commitment'"
    alias    = "!git --no-pager config -l | grep 'alias.' | cut -c7- | awk -F= '{ printf \"\\033\\[32;1m%-20s\\033\\[0m%s\\n\", $1, $2 }'"$2}'"
```


## `git-deploy`

**Is there a better way?** I'm glad you probably asked.

You can put that function's code into an executable script `git-deploy`
in your `PATH` and git will use it when you call `git deploy`.

