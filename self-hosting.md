<!-- hidden-doit -->

# Self hosting

So heroku will stop free plans and surge.sh has a limit for
the size of files you upload.

I use surge.sh to host some static SPAs and heroku as the
API backend.

Surge still works great for smaller deployments but there's
no easy replacement for Heroku's push-to-deploy. Unless you
build your own.


## Time to roll up sleeves

Using your own self hosting stack firstly you avoid surge.sh
free plan's limits and no longer have to wait for Heroku's
free dynos to start. You will lose surge's fast updated CDN
though but we can fix that too.

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
	ndex index.html;
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
  -d exmaple.com \
  -d '*.exmaple.com' \
  --server https://acme-v02.api.letsencrypt.org/directory \
  --register-unsafely-without-email \
  --agree-tos
```

You should use the DNS challenge for it. The cloudflare plugin works
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


## Replacing heroku apps

(Comming soon)


## Replacing heroku addons

(Comming soon)
