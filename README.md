# acme-server-tester

This is a simple CLI tool for testing registering an account against an ACME(RFC_8555) server. This tool is intended for testing purposes only.

Leverages https://github.com/djc/instant-acme

Supports External Account Binding per RFC_8555-7.3.4

# Installation

Several popular installers and binaries are included with each release. Take a look to see if one meets your needs https://github.com/brenden-hogan/acme-server-tester/releases

If not you can compile from source.

## Installing from source

### Install Rust
Install the rust compiler if you haven't done so.

https://www.rust-lang.org/tools/install

### Build
```
cargo build --release
```

If compiling for a different platform then you are building on you may need to look into cross compiling solution like: https://github.com/cross-rs/cross

# Usage

## Arguments

| Arg           | Required| Description |
| --------      | ------- | -------     |
| --server      | Y       | The directory url of the ACME server(RFC_8555-7.1.1). Note: There is no way to set instant-acme to insecure SSL so the servers certificate must be in the callers truststore.|
| --email       | N       | Contact url/email. Typically in the form `mailto:<email>` but other formats that may be accepted are detailed in RFC_8555-7.3.|
| --hmac-key-id | N       | The External Account Binding kid per RFC_8555-7.3.4. If set the hmac-key must also be set or this value will be ignored.|
| --hmac-key    | N       | The External Account Binding HMAC key per RFC_8555-7.3.4 in Base64Url no padding encoding. If set the hmac-key-id must also be set or this value will be ignored. |

## Examples

### With EAB
```
./target/release/acme-server-tester --server https://acme-staging-v02.api.letsencrypt.org/directory --email mailto:<your_email> --hmac-key-id 1 --hmac-key EFABBBAB
```

### Without EAB:
```
./target/release/acme-server-tester --server https://acme-staging-v02.api.letsencrypt.org/directory --email mailto:<your_email>
```

## Expected Output
Account Credentials will be output if successful.
```
account credentials:

{
  "id": "https://acme-staging-v02.api.letsencrypt.org/acme/acct/188487984",
  "key_pkcs8": "<key>",
  "directory": "https://acme-staging-v02.api.letsencrypt.org/directory"
}
```

Otherwise an error will be output like shown below. This is an example from calling a LetsEncrypt Pebble server without EAB when it is set to required by the server.

```
Error: API error: ACME server policy requires newAccount requests must include a value for the 'externalAccountBinding' field (urn:ietf:params:acme:error:externalAccountRequired)
```