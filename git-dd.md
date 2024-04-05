<!-- tags: git, shell -->

# `git dd`

I have this git alias called `dd`. Cool cool. What does it do?

```
$ git help dd
'dd' is aliased to 'deploy'
```

`dd` is a git finger-friendly alias to another git alias.

> "Turtles all the way down"
>
> — Older flat eather

```
$ git help deploy | fold -sw 72
'deploy' is aliased to '!f(){ host=$(git config deploy.host); dir=$(git
config deploy.dir); if [[ $host = '' ]] || [[ $dir = '' ]] ; then echo
'Git config deploy is not configured.'; return 1; fi; git pp; IFS=,;
for h in $host; do echo; echo Updating $(git green $dir) @ $(git green
$h); printf '\033[2m'; git hr; ssh -A $h 'cd '$dir' && git ff && git
log -1'; git hr; printf '\033[0m'; done ;};f'
```

Let's break it down.

- The `!f() { ... ;};f` sets the git alias to call a shell command with
  an inline function `f`, that we call immediately. Inside it:

- Grab the `deploy.host` and `deploy.dir` git config  values.

- Ensure they have values. Exit early with a message if empty.

- `ssh` into the host and rebases the current branch in the target
  directory against its upstream.

The last part is surrounded with a pair of dimmed and reset ansi codes
to signal that output comes from the host, not your local machine.


## Git aliases

Git alias can host not only alias to 1. other git commands and flags,
but also to 2. shelled out calls and, abusing that, 3. inline shell
functions.

Basically we can abuse git as a task manager. And so we shall.


### 1. Subcommand alias

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

The leading `!` again indicates we are shelling out.

`git date` is just an example of calling the `/usr/date` with the
format as the first argument.

Also note how `git parent` calls `git` itself as an external program
and accepts subshell commands (the `$(command)` part). In this case,
it will use the alias we created above `git me` and will resolve the
upstream remote name for the current branch, if it exists.


### 3. Shell functions

```ini
[alias]
    yo = "!f(){ echo "Yo, ${@:-dude}! Have a 🍪" ;};f"
```

The leading `!` tells us we are shelling out, then the `f(){ ... }`
tells we are creating an inline function, and the last `f` means we are
calling it immediately. The extra semicolons are required for shell
one-liners.

`yo` is now an alias for that inline shell function. If you call it,
it will answer.

```
$ git yo
Yo, dude! Have a 🍪
```

If you feel bold, you can pass an argument:

```
$ git yo a monad is a monoid in the category of endofunctors
Yo, a monad is a monoid in the category of endofunctors! Have a 🍪
```

![But why](https://raw.githubusercontent.com/jpedro/jpedro.github.io/master/.github/static/img/why.jpg)

Functions are more flexible that shell commands. Conditionals, loops,
arguments work better. You can download the internets and stream Veep.
From the comfort of your local repo.


## Git Config

Git config is a convenient key value store in
[INI format](https://en.wikipedia.org/wiki/INI_file). You can store
values both in the local repo's `.git/config` or in the global
`~/.gitconfig` file. The command is unsurprising:

    git config ENTRY [VALUE]

The `ENTRY` is formed by an INI `section` and an INI `key` name, joined
by a `.`. If you pass the `VALUE` it sets it. If you don't, it returns
the stored value. For example, a local git repo has this section:

```ini
[branch "master"]
    remote = origin
    merge = refs/heads/master
```

Note how the INI section name has quotes when composed of 2 words.
Which is what we used for the `git parent` alias above. In that case,
the `ENTRY` is `branch.master.remote` and `branch.master` is the INI
section, `remote` being the INI key.

You need to quote aditional section parts if they have dots on them.
For example:

```
$ grep -A 1 hi .git/config
[hi "are.you"]
    ok = "Fine, I'VE NEVER BEEN BETTER"

$ git config hi.are.you.ok
Fine, I'VE NEVER BEEN BETTER
```

There's no section or key checking by git. Just follow your heart, my
friend. You got this!

Armed with this knowledge we can understand now how this unholy
contraption works.


## Ru[n|i]ning things

> "Finger-friendly turtles all the way down"
>
> — Wiser flat eather

If we break down the inline function in the `deploy` alias and format
it humanely and compassionately, it looks like your run-of-the-mill
shell function:

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
f
```

It loads and checks if the `host` and `dir` keys are present in the
`deploy` section and then ssh's into the host and does `git ff` in
that directory. Here's `git ff` along with the rest of the gang:

```ini
[alias]
    pp = "!f(){ git ss \"$1\" && [[ $(git parent) != '' ]] && git push || git push $(git primus) HEAD -u ;};f"
    ff = "!f(){ git fetch $@ && git rebase $(git upstream) || (git rebase --abort && echo '==> Failed to rebase' && exit 1);};f"
    ss = "!f(){ git add --all && git cc \"$1\" ;};f"
    cc = "!f(){ git commit --verbose -m \"${1:-$(git message)}\" || true ;};f"

    parent   = "!git config branch.$(git name).remote"  # Can't use 'remote'
    upstream = "!git rev-parse --abbrev-ref @{u} 2>/dev/null || echo '(none)'"
    origin   = "!git remote get-url origin >/dev/null 2>&1 && echo origin || git remote | head -1"
    message  = "!commitment 2>/dev/null || curl -sfL commit.jpedro.dev || echo 'This reveals a lack of commitment'"
    alias    = "!git --no-pager config -l | grep 'alias.' | cut -c7- | awk -F= '{ printf \"\\033\\[32;1m%-20s\\033\\[0m%s\\n\", $1, $2 }'"$2}'"
```

So not only are we abusing git as a task manager, we are using it as a
code sync mechanism. Fan. Tas. Tic.


## Is there a better way?

I'm glad you probably asked.

You can put that function's code into an executable script `git-deploy`
in your `$PATH` and git will use it when you call `git deploy`.

How does that work? Well, git started as a
[collection of small C binaries](https://github.com/git/git/tree/e83c5163316f89bfbde7d9ab23ca2e25604af290)
named after their commands. For example `git write-tree` used to call
`git-write-tree`. Any executables in your `$PATH` (even better,
[`$GIT_EXEC_PATH`](https://github.com/git/git/blob/c75fd8d8150afdf836b63a8e0534d9b9e3e111ba/exec-cmd.c#L289-L300))
that start with `git-` can be called from git. `git-xxx` can be called
as `git xxx`. I mean, you saved yourself from an hyphen. Now if this
is not something to brag about in the company water cooler, what is?
