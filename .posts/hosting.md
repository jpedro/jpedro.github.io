<!-- hidden-no -->
<!-- tags: shell, linux, kubernetes, reinventing the wheels -->

# Hosting your stuff

I used [surge.sh](https://surge.sh/) to host static content and some
SPAs and [heroku.com](https://www.heroku.com/) as the backend.

But [heroku will stop free plans](https://blog.heroku.com/next-chapter)
and surge [has a 60 MB limit](https://surge.sh) for the total
size of files uploaded. A few larger images and you are toast.

Owning your hosting stack avoids these free plan's limits and no
longer have to wait for Heroku's free dynos to slowly kick in. You will
lose surge's fast updated CDN though but we can fix that too. If you
use Cloudflare you can select which DNS records should be proxied
through them.


## But why?

Fun and profit. Read this: [https://news.ycombinator.com/item?id=39520776#39524350](https://news.ycombinator.com/item?id=39520776#39524350)


## Replacing surge


Replacing surge is the easiest. Just install nginx on a linux machine
and rsync files into the right directory.

**Boom! Done.** Any more questions?


### One conf to rule them all

Nginx's virtual hosts is quite flexible because you can use
`$variables` in places. Nginx 1.9 finally allows variable
interpolation in the `ssl_*` fields. So one `sites-enabled/` conf file
can rule 'em all.

A server block in nginx can look like this:

```
server {
    server_name example.com *.example.com;

    listen 443 ssl http2;
    listen [::]:443 ssl http2;

    ssl_certificate          /etc/letsencrypt/live/$ssl_server_name/fullchain.pem;
    ssl_certificate_key      /etc/letsencrypt/live/$ssl_server_name/privkey.pem;
    ssl_trusted_certificate  /etc/letsencrypt/live/$ssl_server_name/cert.pem;

    root /var/www/vhosts/$host;
    index index.html;
    # ....
}
```

> **Note**
>
> The usage of `$ssl_server_name` comes with a performance
> cost.

So the web root directory for the subdomain `test.example.com` is
found at `/var/www/vhosts/test.example.com`. Transparent.

You just need to symlink `/etc/letsencrypt/live/test.example.com`
to `/etc/letsencrypt/live/example.com` and make sure when you create
the Lets Encrypt cert you add a wildcard domain to it. Something like:

```bash
certbot certonly \
    -d example.com \
    -d '*.example.com' \
    -d '*.acme.example.com' \
    --server https://acme-v02.api.letsencrypt.org/directory \
    --register-unsafely-without-email \
    --agree-tos \
    --dns-cloudflare \
    --dns-cloudflare-credentials cloudflare.ini
```

You should use the DNS challenge for it. The Cloudflare plugin works
well with Lets Encrypt.

Now a "deployment" is an `rsync` call away:

```bash
$ cat ./bin/deploy
#!/usr/bin/env bash
set -euo pipefail

rsync \
    --recursive \
    --archive \
    --verbose \
    --perms \
    --links \
    --compress \
    --delete \
    --exclude '.git' \
    $PWD \
    $(cat CNAME):/var/www/vhosts/$(cat CNAME)

    # "Rave please" is a good mnemonic for the short flags "rsync -ravplz"
```

Easy. And yes. That `CNAME` contains the single line `test.example.com`,
just like surge.


### Todos

- Introduce surge-like directives, like `redirect` and `auth`, that
  basically render a custom nginx `sites-enabled/` conf files for a
  deployment. You lose the single conf to rule 'em all but you can
  optimize settings per host. Caching static assets could be one of
  them, if you are not proxying those through some CDN.


## Replacing heroku apps

The idea is to replace heroku's build packs with some tool that knows
how to package the repo files into a docker image and deploy it.

The tool figures out what type of application the repo is and how it
should be handled or you explicitly tell it to.

Kubernetes or plain old docker daemon can fit neatly as the container
runtime.


### Implicit

The laziest approach to guessing how the repo should be packaged is to
look for some files that indicate which language, framework the repo
uses.

In java one can check if there's a `pom.xml` or a `build.gradle` file.
For go that would be a `go.mod`, for rust `Cargo.toml`.

That informs the tool which base docker image should use, copies the
repo files into it, install dependencies, run tests in the staged
build container, copy the relevant built files into the final stage
and deploys that via a rendered kubernetes, docker-composer or helm
charts.


### Explicit

If you don't need the latest and greatest default configuration, you
can tell the script what to do. This could look like:

```yaml
test:
  command: go test
  coverage: true # or configure a coverage.out file path

build:
  command: go build .
  image:
    base: golang:latest
    name: hello-kaiku
    tags:
    - latest
    - time-$(( build.time.now ))
    - version-$(( build.git.describe ))

deploy:
  targets:
  - docker-compose
  - kubernetes
  - helm

run:
  # These get translated into `spec.template.spec.containers[]` fields
  # Overrides the command in the docker base image
  command: ['sh', '-c', 'echo "Hello, Kubernetes!" && sleep 3600']
  env:
    SOME_VAR: some-value
    SOME_SECRET: $(( build.secrets.SOME_SECRET ))
    LISTEN_POST: $(( run.PORT ))
```

Here the use of `$(( scope.var ))` is used to dynamically insert values
at different phases.

The configured `test`, `build`, and `run` stages above have defaults
for each runtime the tool figures out and merges.


### Todos

- Based on the `CNAME` file or some configuration, ensure the required
  TLS certificates exist and get attached to the kubernetes Ingress.


## Replacing heroku addons

This boils down to exposing some CRUD API that creates cloud resources,
like a new postgres database, a redis database or an elasticsearch
index and exposes environment variables for each deployment that uses
them.


### Todos

- There should be a way to uniquely identify a repo so each time it
  chages its name or domain the same resources env gets injected into
  the kubernetes deployment via config maps or secrets.
