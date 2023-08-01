# My favourite git alias

My current favourite git alias is `dd`.

What does it do? Let's ask git.

    $ git help dd
    'dd' is aliased to 'deploy'

O... kay. What *is* `git deploy` then?

    $ git help deploy
    'deploy' is aliased to '!f(){ host=$(git config deploy.host); dir=$(git config deploy.dir); if [[ $host = '' ]] || [[ $dir = '' ]] ; then echo 'Git config deploy is not configured.'; return; fi; git pp; echo '\033[2m'; ssh -A $host 'cd '$dir' && git ff && git log -1'; echo '\033[0m' ;};f'

That's quite a bit to digest. Let's break it down in 4 parts:

1. The `!f() { ... ;};f` sets the git alias to become a shell
    function. Inside this function we:

2. Grab the `deploy.host` and `deploy.dir` git config  values.

3. Ensure they are not empty. We exit early with a message if empty.

4. Run an `ssh` command.


## Git aliases

You read it. Git alias can host not only alias to 1. other git commands
and flags, not only to 2. shell calls but also 3. shell functions.


1. **Command alias**

    ```ini
    [alias]
    st = status
    me = branch --show-current
    ```

So now `git st` is short for `git status`. And `git me` is short for
`git branch --show-current` (or if you are using an older git version
set it to `rev-parse --abbrev-ref HEAD`).


2. **Shell calls**

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
it will find the upstream remote name for the current branch, if it
exists.


3. **Shell functions**

```ini
[alias]
yo = "!f(){ echo "Yo ${@:-dude}!" ;};f"
```

`git yo` is now an alias for a shell function that we just created.
If you call `git yo` it will print:

```bash
$ git yo
Yo dude!
```

And if you pass an argument:

```bash
$ git yo alrighty
Yo alrighty!
```


## Git config

You can store both local repo and global configuration values in git.
The command is quite unsurprisingly `git config [KEY] [VALUE]`. To read
an entry just pass the `KEY`. To set an entry pass the `VALUE`.
<!-- START FOOTER -->
 &nbsp;

<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js"></script>
<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-database.js"></script>
<script src="https://jpedro.github.io/js/v1/data.js"></script>
<script src="https://jpedro.github.io/js/v1/comments.js"></script>
<script defer="">Comments.mount(document.body.children[0]);</script>
<!-- END FOOTER -->
