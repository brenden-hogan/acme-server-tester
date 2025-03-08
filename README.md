# acme-server-tester

This is a simple CLI tool for testing registering an account against an ACME(RFC 8555) server. This tool is intended for testing purposes only.

Leverages https://github.com/djc/instant-acme

Supports External Account Binding

# Usage

## Install
Install the rust compiler if you haven't done so.

https://www.rust-lang.org/tools/install

## Build
```
cargo build --release
```

Note if compiling for a different system then the one you are building on you'll have to set the `--target` flag according to https://doc.rust-lang.org/cargo/appendix/glossary.html#target

ex:
```
cargo build --release --target x86_64-unknown-linux-gnu
```

See other options at https://doc.rust-lang.org/rustc/platform-support.html

## Run


With EAB
```
./target/release/acme-server-tester --server https://acme-staging-v02.api.letsencrypt.org/directory --email mailto:<your_email> --hmac-key-id 1 --hmac-key EFABBBAB
```

Note: `hmac-key` is the Base64 URL Encoded Key.

Without EAB:
```
./target/release/acme-server-tester --server https://acme-staging-v02.api.letsencrypt.org/directory --email mailto:<your_email>
```

## Expectation
Account Credentials will be output if successful:
```
account credentials:

{
  "id": "https://acme-staging-v02.api.letsencrypt.org/acme/acct/188487984",
  "key_pkcs8": "<key>",
  "directory": "https://acme-staging-v02.api.letsencrypt.org/directory"
}
```