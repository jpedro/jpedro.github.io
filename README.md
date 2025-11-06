# Posts nobody asked for

![Everything is fine](https://img.shields.io/badge/This_shit_is_amazing-Even_more_amazing:_you_are_reading_this!-brightgreen)


## Features

Add a markdown comment in the format `<!-- write something smart here -->`
to set properties. Supported:

| Comment    | Description                                 | Type    | Default  |
| ---------- | ------------------------------------------- | ------- | -------- |
| `hidden`   | Hides a page from the `index.md`            | boolean | `false`  |
| `title`    | Overrides `# H1` and beautified file name   | string  | `""`     |
| `tags`     | Custom tags                                 | csv     | `""`     |
| `comments` | Controls comments in pages (only dad jokes) | boolean | `false`  |


## Todos

- [x] Move posts from the root directory to [.posts](.posts).
- [x] Dump generated posts into the [docs](docs) directory.
- [ ] Add generated tags at the top of each page.
- [ ] Allow multi-line comments.
- [ ] Allow `index` and `date` ordering (in that order).
- [ ] Allow `path` for custom urls.
