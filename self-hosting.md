<!-- hidden-doit -->

# Self hosting

So heroku [will stop free plans](https://blog.heroku.com/next-chapter)
and [surge.sh](https://surge.sh) has a limit for the size of files
you upload.

I use surge.sh to host some static SPAs and heroku as the
API backend.

Surge still works great for smaller deployments but there's
no easy replacement for Heroku's push-to-deploy. Unless you
build your own.


## Replacing surge

Using your own self hosting stack firstly you avoid surge.sh
free plan's limits and no longer have to wait for Heroku's
free dynos to start. You will lose surge's fast updated CDN
though but we can fix that too. If you use Cloudflare, just
set the static content subdomain(s) and proxy that through them.

Replacing surge is the easiest. Just install nginx on a
linux machine and run rsync to the right directory.

Nginx's virtual hosting is quite flexible because you can use
`$<var>` in most of their fields. Nginx 1.9 finally allows
variable interpolation in the `ssl_*` fields. So one
`sites-enabled/` config file can rule them all.

A server block in nginx looks like this:

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
> Note that the usage of `$ssl_server_name` cames with a performance price.

So the web root directory for the subdomain `test.example.com` is
found at `/var/www/vhosts/test.example.com`. Transparent.

You just need to symlink `/etc/letsencrypt/live/test.example.com`
to `/etc/letsencrypt/live/example.com` and make sure when you create
the Lets Encrypt cert, you add a wildcard domain to it. Something like:

```bash
certbot certonly \
    -d example.com \
    -d '*.example.com' \
    --server https://acme-v02.api.letsencrypt.org/directory \
    --register-unsafely-without-email \
    --agree-tos
```

You should use the DNS challenge for it. The Cloudflare plugin works
really well with Lets Encrypt.

Now a "deployment" is an `rsync` call away:

```bash
$ cat ./bin/deploy
#!/usr/bin/env bash
set -euo pipefail

rsync \
    --verbose \
    --archive \
    --links \
    --compress \
    --recursive \
    --delete \
    --exclude '.git' \
    $PWD \
    $(cat CNAME):/var/www/vhosts/$(cat CNAME)
```

Easy. And yes. That `CNAME` contains the single line `test.example.com`,
just like surge.

### Todos

- [ ] Introduce surge-like directives, like redirects and auth, that
      basically re-render nginx `sites-enabled/` conf files for a
      deployment.


## Replacing heroku apps

To keep things simple, the idea here is to replace heroku's build packs
with some script or tool that packages the repo files into a docker
image and deploys it to some container runtime.

The script either intelligently figure out what type of application the
repo runs and how it should package it and deploy it or one explicitly
tell it.

Kubernetes fits neatly as the container runtime.


### ~Intelligent~ Lazy guesses

The laziest approach to guessing how the repo should be packaged is to
look for some files that indicate which language, framework and
dependencies it should use.

In java one can check if there's a `pom.xml` or a `build.gradle`. For
go that would be a `go.mod`.

That would tell the script which base docker image should use, copy the
repo files into it, install dependencies, run tests in the staged build
container, copy the relevant built files into the final stage and deploy
that via a rendered kubernetes templates or configure a helm chart.


### Explicit

If you don't need the latest and greatest default runtime configuration,
you can use a deployment file to explicitly tell the script what to do.
This could look like:

```yaml
test:
  command: go test
  coverage: true # or configure a coverage.out file path

build:
  image: golang:latest
  args: []
  name: hello-kaiku
  tags:
  - latest
  - time-$(( build.time.now ))
  - version-$(( build.git.describe ))

run:
  # These get translated into `spec.template.spec.containers[]` fields
  # Overrides the command in the docker base image
  command: ['sh', '-c', 'echo "Hello, Kubernetes!" && sleep 3600']
  env:
    SOME_VAR: some-value
    SOME_SECRET: $(( build.SOME_SECRET ))
    LISTEN_POST: $(( run.PORT ))
```

Here the use of `$(( scope.var ))` is used to dynamically insert values
at different phases.

The configured `test`, `build`, and `run` phases above have defaults
for each runtime the tool figure out and get merged.


### Todos

- [ ] Based on the `CNAME` file or some other configuration, create the
      required TLS certificates. And, of course, attach those to the
      kubernetes Ingress.


## Replacing heroku addons

Here the challenge boils down to exposing some CRUD API that creates
cloud resources, like a new postgres database, a redis database and
exposes some environment variables via a file for each deployment made.

There should be a way to uniquely identify a repo so each time it
chages its name the same resources env gets attached.
