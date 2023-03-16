[![Build](https://github.com/c0deWranglr/secret-share/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/c0deWranglr/secret-share/actions/workflows/build-and-release.yml)

# secret-share

A secure way to share a secret or password with others. The source code is provided open to ensure public accountability. Deploy it to your liking or use the hosted version can be found at https://passdl.net.

## Server-Side Storage

Only an encrypted version of the secret is transferred over the internet. The client browser handles all encryption/decryption which leaves the server with **zero ability** to reverse engineer any of the secrets it stores. 

Each encrypted secret is stored in a randomly selected slot, which becomes the key that is returned to the client browser. This key is randomly generated and not in any way associated with the encrypted secret. 

Furthermore, each encrypted secret is required to have an expiration time. Which limits how long the encrypted secret is available across the network. Optionally, a maximum number of attempts can be specified to prematurely lock the encrypted secret from being accessed.

Before storing the secret, the server encrypts the secret along with its expiration metadata using an encryption key known only to the server. This extra step helps protect the data at rest.

_As of right now, expiration is done lazily. This means that the server can hold onto secrets after they've expired until they are accessed again._

### Storage Adapters

`StorageAdapter`s are plugins for choosing where to store a secret. They don't contain any security/encryption details about the information they're storing. The currently supported adapters are:

1. [In Memory](/server/src/storage/adapters/in_memory.rs) - Useful in a stateful environment for short lived secrets
2. [Google Cloud Storage](/server/src/storage/adapters/google_cloud_storage.rs) - Useful in a stateless GCP environment for longer lived secrets

## Running

```
make release start-docker
```

Additional Makefile rules:
- `start-client` - Start the client in development mode
- `start-server` - Start the server in development mode
- `start-docker` - Start a docker container built via `release`, set `PORT` to customize the exposed docker port
- `release-client` - Build a production version of the client
- `release-server` - Build a production version of the server 
- `release-docker` - Build a docker image using the release versions of the server and client
- `release` - Run all of  the release rules

## Client-Side Encryption

The most important part of this project is what happen on the client browser. Since the server is essentially an encrypting key-value store, the client is the driving force in deciding what is actually stored. 

The client uses AES encryption via [CryptoJS](https://cryptojs.gitbook.io/docs/#ciphers). To encrypt, the client requires user specified values for the following:

1. Passowrd / Secret - The data that will be encrypted
2. Token - A shareable value that will be used as the encryption key. 

Given a secret and a token, the following steps are undergone:

1. Hash the token (SHA256) 100x - 100 is an arbitrary number. The purpose here is just to create a unique non reversable encryption key
2. Encrypt the secret (AES) using the hashed token
3. Send the output to the server along with additional request params (Expiration, Max Attempts, etc)

When the encrypted secret is retrieved from the server for viewing, it requires the same token be entered for decryption.

## Screenshots

![](/save_view.png)

Save

<br>
<br>

![](/view_view.png)

View