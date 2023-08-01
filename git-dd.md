<!-- tags: git, shell -->
# `git-dd`

My current favourite git alias is `dd`.

What does it do? Let's ask git.

```bash
$ git help dd
'dd' is aliased to 'deploy'
```

O... kay. That was... *helpful*... but what's `git deploy` then?

It's another alias. `dd` is a git finger-friendly alias to yet another
git alias.

> "Turtles all the way down"
>
> — Older wiser flat eather

```bash
$ git help deploy
'deploy' is aliased to '!f(){ host=$(git config deploy.host); dir=$(git config deploy.dir); if [[ $host = '' ]] || [[ $dir = '' ]] ; then echo 'Git config deploy is not configured.'; return; fi; git pp; echo '\033[2m'; ssh -A $host 'cd '$dir' && git ff && git log -1'; echo '\033[0m' ;};f'
```

That's quite a bit to digest. Let's break it down in 4 parts:

1. The `!f() { ... ;};f` sets the git alias to become a shell
    function. Inside this function we:

2. Grab the `deploy.host` and `deploy.dir` git config  values.

3. Ensure they are not empty. We exit early with a message if empty.

4. Run some `ssh` command.


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
parent = "!git config branch.$(git me).remote" # Can't call it `remote`
```

The `!` indicate this is a external shell call.

`git date` is just an example of calling the `/usr/date` with the
format as the first argument.

Also note how `git parent` calls `git` itself as an external program
and accepts subshell commands (the `$(command)` part). In this case,
it will use the alias we created above `git me` and will resolve the
upstream remote name for the current branch, if it exists.


### 3. Shell functions

```ini
[alias]
yo = "!f(){ echo "Yo, ${@:-dude}!" ;};f"
```

`git yo` is now an alias for an inline shell function that we just
created. If you call `git yo` it will utter:

```bash
$ git yo
Yo, dude!
```

If you feel bold, you pass an argument:

```bash
$ git yo a monad is a monoid in the category of endofunctors
Yo, a monad is a monoid in the category of endofunctors!
```

![But why](https://raw.githubusercontent.com/jpedro/jpedro.github.io/master/.github/static/img/why.jpg)

Why? Because shell functions are more flexible. You can do `if`s and
`for` loops, default arguments, download the internets and stream Veep.
As your heart always wanted to but just didn't know how.


## Git config

Git config is a key value store in
[`INI` format](https://en.wikipedia.org/wiki/INI_file).

You can store values both in the local repo's `.git/config` or in the
global `~/.gitconfig` file. The command is unsurprisingly
`git config [ENTRY] [VALUE]`. The INI `ENTRY` is formed by a section
and a key name. For example my local git repo has this entry:

```ini
[branch "master"]
  remote = origin
  merge = refs/heads/master
```

Which is what we use for the `git parent` alias above. In that case the
`ENTRY` is `branch.master.remote` and `branch.master` is the section,
`remote` being the key.

You need to quote section name if they are further "dot" separated.
For example:

```bash
$ grep -A 1 hi .git/config
[hi "are.you"]
  ok = "YES, I'VE NEVER BEEN BETTER!!!"

$ git config hi.are.you.ok
YES, I'VE NEVER BEEN BETTER!!!
```

To read an entry pass only the `ENTRY`. To set an entry pass both
`ENTRY` and `VALUE`.

And you are not limited to git's internal known values. You can add
your own.

Armed with this knowledge we can understand now how this unholy
`git deploy` contraption works.



## Git deploy

> "Turtles all the way down"
>
> — Older wiser flat eather



<!-- START FOOTER -->
 &nbsp;

<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js"></script>
<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-database.js"></script>
<script src="https://jpedro.github.io/js/v1/data.js"></script>
<script src="https://jpedro.github.io/js/v1/comments.js"></script>
<script defer="">Comments.mount(document.body.children[0]);</script>
<!-- END FOOTER -->
