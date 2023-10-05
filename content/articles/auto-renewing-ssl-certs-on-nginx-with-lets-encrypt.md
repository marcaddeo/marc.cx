---
title: <s>Auto Renewing SSL Certs on NGINX with Let's Encrypt</s>
clean_title: Auto Renewing SSL Certs on NGINX with Let's Encrypt
slug: auto-renewing-ssl-certs-on-nginx-with-lets-encrypt
status: published
published: 2015-12-12 00:00
excerpt: How I'm using Let's Encrypt for free auto renewing SSL certificates on NGINX
tags:
  - linux
---

> [!WARNING]
> This article is out of date and should no longer be used.

# Auto Renewing SSL Certs on NGINX with Let's Encrypt

[Let's Encrypt][] has recently entered a public beta and I've been really excited to test it out. They're offering free SSL certs that expire every 90 days, and are extremely easy to get. Having used Comodo in the past to get certificates, I've been waiting for this day.

And it was everything I'd hoped it to be. The first cert I installed took minutes! I tried the `letsencrypt-auto` client at work to get a cert for one of our internal domains. But I wasn't able to find an easy way to automate the `letsencrypt-auto` and didn't really like the fact that it generated configuration files automagically.

If you're using [Let's Encrypt][], I highly recommend you [donate][] to help further it's development!

## Let's Encrypt Client

Lukcily for me, [Let's Encrypt][] is really a set of protocols that allow a client to interface with their ACME auth server to generate certficiates automatically.

And there just so happend to be an alternative client, made by one of the core LE developers, called [simp_le][] that better met my needs:

1. Allow me to easily script and automate renewal
2. Don't touch configuration files

### Installing simp_le

Installing the [simp_le][] client is easy.

```bash
$ cd /opt
$ git clone https://github.com/kuba/simp_le.git
$ cd simp_le
$ ./bootstrap.sh
$ ./venv.sh
$ ln -s /opt/venv/bin/simp_le /usr/local/sbin/simp_le
```

## Generating our first cert

Generating an SSL with [Let's Encrypt][] and [simp_le][] is super easy. I'm orgnaizing my certs like so:

```bash
/var/cert
├── marc.cx
│   ├── .simp_le_renew.json
│   ├── account_key.json
│   ├── chain.pem
│   ├── fullchain.pem
│   └── key.pem
└── subdomain.marc.cx
│   ├── .simp_le_renew.json
    ├── account_key.json
    ├── chain.pem
    ├── fullchain.pem
    └── key.pem
```

And generating them like so:

```bash
$ cd /var/cert/marc.cx
$ simp_le \
    --email hi@marc.cx \
    -f account_key.json \
    -f fullchain.pem \
    -f chain.pem \
    -f key.pem \
    -d marc.cx:/var/www/marc.cx/public_html
```

The last last argument of the command specifies the domain and the webroot as `domain:webroot`. This is important because the client needs to create a file in the webroot that can be used to verify domain ownership.

Since this site is build using the [Phoenix Framework][] and the [Elixir Language][], I had to tweak my `nginx` config a little bit to serve the `/.well-known/acme-challenge` file for domain control validation.

```nginx
upstream marc_cx {
    server 127.0.0.1:8080;
}

server {
    server_name marc.cx;
    listen 80 default;
    listen [::]:80 default;

    location '/.well-known/acme-challenge' {
        default_type 'text/plain';
        root /var/www/marc.cx/public_html;
    }

    location / {
        try_files $uri @proxy;
    }

    location @proxy {
        include proxy_params;
        proxy_redirect off;
        proxy_pass http://marc_cx;
    }
}
```

That's it! A new cert will have been generated in `/var/cert/marc.cx`. Next, `nginx` needs to be configured for ssl.

## Configuring NGINX to use SSL

To do this, I used the modern configuration option on the [Mozilla SSL Configuration Generator][]. My configuration ended up looking like this.

```nginx
upstream marc_cx {
    server 127.0.0.1:8080;
}

server {
    server_name marc.cx;
    listen 80 default;
    listen [::]:80 default;

    location '/.well-known/acme-challenge' {
        default_type 'text/plain';
        root /var/www/marc.cx/public_html;
    }

    return 301 https://$server_name$request_uri;
}

server {
    server_name marc.cx;
    listen 443 default ssl;
    listen [::]:443 ipv6only=on ssl;

    ssl_certificate /var/cert/marc.cx/fullchain.pem;
    ssl_certificate_key /var/cert/marc.cx/key.pem;

    ssl_session_timeout 1d;
    ssl_session_cache shared:SSL:50m;
    ssl_session_tickets off;

    # Diffie-Hellman parameter for DHE ciphersuites, recommended 2048 bits
    # $ openssl dhparam -out /etc/nginx/dhparam.pem 2048
    ssl_dhparam /etc/nginx/dhparam.pem;

    # modern configuration. tweak to your needs.
    ssl_protocols TLSv1.1 TLSv1.2;
    ssl_ciphers 'ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-DSS-AES128-GCM-SHA256:kEDH+AESGCM:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA:ECDHE-ECDSA-AES256-SHA:DHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA:DHE-DSS-AES128-SHA256:DHE-RSA-AES256-SHA256:DHE-DSS-AES256-SHA:DHE-RSA-AES256-SHA:!aNULL:!eNULL:!EXPORT:!DES:!RC4:!3DES:!MD5:!PSK';
    ssl_prefer_server_ciphers on;

    # HSTS (ngx_http_headers_module is required) (15768000 seconds = 6 months)
    add_header Strict-Transport-Security max-age=15768000;

    # OCSP Stapling ---
    # fetch OCSP records from URL in ssl_certificate and cache them
    ssl_stapling on;
    ssl_stapling_verify on;

    ## verify chain of trust of OCSP response using Root CA and Intermediate certs
    ssl_trusted_certificate /var/cert/marc.cx/chain.pem;

    resolver 8.8.8.8 8.8.4.4;

    location / {
        try_files $uri @proxy;
    }

    location @proxy {
        include proxy_params;
        proxy_redirect off;
        proxy_pass http://marc_cx;
    }
}
```

This is also configured to redirect any http connections to https.

## Testing SSL
We should then test to make sure we're actually secure. We can use the [Qualys SSL Labs SSL Tester][].

![SSL Grade][]

Everything looks good! Now to automate the renewal process!

## Automating Certficate Renewal
In order to automatically renew LE SSL certs, I've written a simple wrapper for [simp_le][] that uses a simple configuration file to allow my script to know the sites webroot.

This script can, and should, be configured using the following environment variables:

* `LE_EMAIL` is the email used for [Let's Encrypt][]. Defaults to `deveops@$HOST` where `$HOST` is the machines hostname.
* `SIMP_LE_CERT_PATH` is where the cert files are stored and organized as shown above. Defaults to `/var/cert`.
* `SIMP_LE_CONF_FILE_NAME` is the name of the configuration file to determine a sites webroot. Defaults to `.simp_le_renew.json`.
* `SIMP_LE_WEB_SERVER_RESTART_COMMAND` is the command that is issued to restart the webserver once a cert has been successfully renewed. Defaults to `service nginx restart` and can easily be changed to support apache e.g. `service apache2 restart`.

### simp_le_renew wrapper for simp_le

```bash
#!/bin/bash
LE_EMAIL="${LE_EMAIL:-devops@$HOST}"
SIMP_LE=/usr/local/sbin/simp_le
SIMP_LE_CERT_PATH="${SIMP_LE_CERT_PATH:-/var/cert}"
SIMP_LE_CONF_FILE_NAME="${SIMP_LE_CONF_FILE_NAME:-.simp_le_renew.json}"
SIMP_LE_WEB_SERVER_RESTART_COMMAND="${SIMP_LE_WEB_SERVER_RESTART_COMMAND:-service nginx restart}"

function check_requirements {
    hash jq 2>/dev/null || {
        echo >&2 "\`jq\` is required for this script to run! Aborting..."
        exit 1
    }

    if [[ "$EUID" -ne 0 ]]; then
        echo >&2 "This script must be run as root. Aborting..."
        exit 1
    fi
}

function get_conf_files {
    echo $(find $SIMP_LE_CERT_PATH -type f -name "$SIMP_LE_CONF_FILE_NAME")
}

function renew_cert() {
    cd $1
    ($SIMP_LE \
        --email $LE_EMAIL \
        -f account_key.json \
        -f chain.pem \
        -f fullchain.pem \
        -f cert.pem \
        -f key.pem \
        -d $2:$3 \
    && $SIMP_LE_WEB_SERVER_RESTART_COMMAND) || true
}

function renew_certs {
    for FILE in $(get_conf_files); do
        local CERT_PATH="$(dirname $FILE)"
        local DOMAIN="$(basename $CERT_PATH)"
        local WEBROOT="$(cat $FILE | jq -r '.webroot')"

        renew_cert $CERT_PATH $DOMAIN $WEBROOT
    done
}

check_requirements
renew_certs
```

### Setting Up a Cronjob

This is the entry I have in my crontab to check and renew certs everyday at `2am`:

```bash
0 2 * * * LE_EMAIL=hi@marc.cx /usr/local/sbin/simp_le_renew >> /var/log/simp_le_renew.log 2>&1 || true
```

## Closing Words

And that's it! It's pretty simple to get up and running with [Let's Encrypt][]! And being able to easily automate the renewal process with [simp_le][] and [simp_le_renew][] relieves the headache of remembering to renew your SSL certificates and having a potential website outage!

[Let's Encrypt]: https://letsencrypt.org/
[donate]: https://letsencrypt.org/become-a-sponsor/
[simp_le]: https://github.com/kuba/simp_le
[Phoenix Framework]: http://www.phoenixframework.org/
[Elixir Language]: http://elixir-lang.org/
[Mozilla SSL Configuration Generator]: https://mozilla.github.io/server-side-tls/ssl-config-generator/
[Qualys SSL Labs SSL Tester]: https://www.ssllabs.com/ssltest/
[SSL Grade]: https://i.imgur.com/DpUQ3FD.png
[simp_le_renew]: https://github.com/marcaddeo/simp_le_renew

#blog/devops