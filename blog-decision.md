# Blog decision

I started a few blogs but failed. Not knowing what to write was the problem.
Also, the right tool was not obvious for quite a while.

Many people highlighted the value of writing down stuff. It requires some time
but I'll try this now.


## Enter GitHub

I've been collecting notes on GitHub repos and there's of course gists. But
the 2 things that changed my opinion to use GitHub as the static blog hosting
were:

1. GitHub uses the `github.com/USER/USER.github.io` repo as source for
   [USER.github.io](https://USER.github.io). Which is pretty neat.

2. GitHub actually renders markdown pages directly. So no need to add a theme
   and an engine transforming markdown into HTML. And the colors and layout are
   decent enough to work. The index page for markdown is simply `README.md`.


## Todos

- [ ] Add metadata (via comments) to pages, like tags, summary, created etc...
- [ ] Have a script running from GitHub actions that generates the `tags/`
      index page.
- [ ] Add a comments section on each page.
