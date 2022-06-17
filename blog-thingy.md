# Blog Thingy

I tried to start a few blogs but failed. Not knowing what to write was
one problem. Also, the right tool was not obvious for quite a while.

The internets keep saying me the value of writing down stuff. It requires
some time but I'll try this now.

It's easy, and most importantly, it's cheap. Free actually. As in free
from maintenance. And now I've (finally) one thing to write about.


## Enter github.io

I've been collecting notes on a GitHub repo and gists. But the 2 things
that made GitHub a good static blog hosting were:

1. GitHub uses the `github.com/USER/USER.github.io` repo as source for
   [USER.github.io](https://USER.github.io). Which is handy. My blog is
   my GH account.

2. GitHub does a decent job of rendering markdown files. So no need to
   find and having a whole engine, rendering markdown into HTML.

   The first page for the rendered github.io markdown is simply
   `index.md` and if that doesn't exist, `README.md`. This distinction
   is welcomed as one can be used internally for the repo and `index.md`
   could be generated.


## Todos

- Add metadata (via comments) to pages, like tags, summary, created etc.
- Have a script running from a GitHub workflow that generates the
  `tags/` index page. And even extracts a summary. Because lazy.
- Think how to add a comments section on each rendered page because
  markdown doesn't allow javascript.
