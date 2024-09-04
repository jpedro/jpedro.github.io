# Posts nobody asked for

![Everything is fine](https://img.shields.io/badge/This_shit_is_amazing-Even_more_amazing:_you_are_reading_this!-brightgreen)


## Features

Add a markdown comment in the format `<!-- write something smart here -->`
to set properties. Supported:

| Comment    | Description                                            | Type    | Default         |
| ---------- | ------------------------------------------------------ | ------- | --------------- |
| `hidden`   | Hides a page from the `index.md`                       | boolean | `false`         |
| `title`    | Overrides both `# H1` and beautified file name         | string  | `""`            |
| `tags`     | Custom tags                                            | csv     | `shell, linux`  |
| `comments` | Controls comments in pages (right now, only dad jokes) | boolean | `true`, `false` |



## Todos

- [x] Move posts from the root directory to [.posts](.posts)
- [ ] Add generated tags at the top of each page.
- [ ] Allow multi-line comments.
- [ ] Allow `date` and `index` ordering.
- [ ] Allow `path` or the ugly noun `slug` for custom urls.
