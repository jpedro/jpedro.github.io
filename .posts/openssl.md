<!-- tags: shell, security -->
<!-- hidden -->

# Useful OpenSSL commands

[Original post](https://www.sslshopper.com/article-most-common-openssl-commands.html)

One of the most versatile SSL tools is OpenSSL which is an open source
implementation of the SSL protocol. There are versions of OpenSSL for
nearly every platform, including Windows, Linux, and Mac OS X. OpenSSL
is commonly used to create the CSR and private key for many different
platforms, including Apache. However, it also has hundreds of different
functions that allow you to view the details of a CSR or certificate,
compare an MD5 hash of the certificate and private key (to make sure
they match), verify that a certificate is installed properly on any
website, and convert the certificate to a different format.

If you don't want to bother with OpenSSL, you can do many of the same
things with our SSL Certificate Tools. Below, we have listed the most
common OpenSSL commands and their usage:


## General OpenSSL Commands
These commands allow you to

- Generate CSRs
- Generate certificates
- Generate private keys
- Other miscellaneous tasks.


#### Generate a new private key and Certificate Signing Request

    openssl req \
        -new \
        -newkey rsa:2048 \
        -nodes \
        -keyout private.key \
        -out request.csr \


#### Generate a self-signed certificate

    openssl req \
        -x509 \
        -sha256 \
        -nodes \
        -days 365 \
        -newkey rsa:2048 \
        -keyout private.key \
        -out certificate.crt


#### Generate a certificate signing request (CSR) for a private key

    openssl req \
        -new \
        -key private.key \
        -out request.csr


#### Generate a certificate signing request based on a certificate

    openssl x509 \
        -x509toreq \
        -signkey private.key \
        -in certificate.crt \
        -out request.csr


#### Remove a passphrase from a private key

    openssl rsa \
        -in private.pem \
        -out new-private.pem


## Checking Using OpenSSL

If you need to check the information within a Certificate, CSR or
Private Key, use these commands. You can also check CSRs and
check certificates using our online tools.


#### Check a Certificate Signing Request (CSR)

    openssl req -text -noout -verify -in CSR.csr


#### Check a private key

    openssl rsa -in private.key -check


#### Check a certificate

    openssl x509 -in certificate.crt -text -noout


#### Check a PKCS#12 file (.pfx or .p12)

    openssl pkcs12 -info -in keyStore.p12


## Debugging Using OpenSSL

If you are receiving an error that the private doesn't match
the certificate or that a certificate that you installed to
a site is not trusted, try one of these commands. If you are
trying to verify that an SSL certificate is installed correctly,
be sure to check out the SSL Checker.


#### Check an MD5 hash of the public key to ensure that it matches with what is in a CSR or private key

    openssl x509 -noout -modulus -in certificate.crt | openssl md5
    openssl rsa -noout -modulus -in privateKey.key | openssl md5
    openssl req -noout -modulus -in CSR.csr | openssl md5


#### Check an SSL connection

All the certificates (including Intermediates) should be displayed

    openssl s_client -connect www.paypal.com:443


## Converting Using OpenSSL

These commands allow you to convert certificates and keys to
different formats to make them compatible with specific types
of servers or software. For example, you can convert a normal
PEM file that would work with Apache to a PFX (PKCS#12) file
and use it with Tomcat or IIS. Use our SSL Converter to convert
certificates without messing with OpenSSL.


#### Convert a DER file (.crt .cer .der) to PEM

    openssl x509 -inform der -in certificate.cer -out certificate.pem


#### Convert a PEM file to DER

    openssl x509 -outform der -in certificate.pem -out certificate.der


#### Convert a PKCS#12 file (.pfx .p12) containing a private key and certificates to PEM

    openssl pkcs12 -in keyStore.pfx -out keyStore.pem -nodes

You can add -nocerts to only output the private key or add -nokeys to only output the certificates.


#### Convert a PEM certificate file and a private key to PKCS#12 (.pfx .p12)

    openssl pkcs12 -export -out certificate.pfx -inkey privateKey.key -in certificate.crt -certfile CACert.crt
