# Self hosting

So heroku will stop free plans and surge.sh has a limit for
the size of all combined the files you upload to them.

I use surge.sh to host some small static SPAs using heroku
apps as the API backend.

Surge still works great for smaller deployments but there's
no easy replacement for Heroku's push-to-deploy.


## Time to roll up your own

"Every problem is an opportunity in disguise." -- someone

In this case, you avoid surge.sh free plan's file size limits
and no longer have to wait for Heroku's free dynos to start.
You will lose surge's fast updated CDN though but we can
replace that too.

Replacing surge is by the easiest. Just install nginx on a
Debian host and run rsync to the right directory.

Nginx's virtual hosting is simple because you can use $vars
in most of their fields. Nginx 1.9 finally allowed variable
interpolation in the `ssl_*` fields. So one `site-enabled`
can rule them all.

A "dynamic" server block in nginx looks like this now:

```
server {
	server_name example.com *.example.com;

	listen 443 ssl http2;
	listen [::]:443 ssl http2;

	ssl_certificate          /etc/letsencrypt/live/$host/fullchain.pem;
	ssl_certificate_key      /etc/letsencrypt/live/$host/privkey.pem;
	ssl_trusted_certificate  /etc/letsencrypt/live/$host/cert.pem;

	root /var/www/vhosts/$host;
  index index.html;
  # ....
}
```

So the web root directory for the subdomain `test.example.com` is
found at `/var/www/vhosts/test.example.com`. Transparent.

You just need to symlink `/etc/letsencrypt/live/test.example.com`
to `/etc/letsencrypt/live/example.com` and make sure when you create
the Lets Encrypt cert, you add a wildcard domain to it. Something like:

```
certbot certonly \
  -d exmaple.com \
  -d '*.exmaple.com' \
  --server https://acme-v02.api.letsencrypt.org/directory \
  --register-unsafely-without-email \
  --agree-tos
```

You should use the DNS challenge for it. The cloudflare plugin works
really well with Lets Encrypt.

Now a dpeloyment is an `rsync` call away:

```
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

Easy.

Note: Yes. That `CNAME` contains the single line `test.example.com`,
as surge uses it.


## Replacing heroku

(Comming soon)
