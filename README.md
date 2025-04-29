# cargo-credential-jfrog

A simple cargo [credential provider](https://doc.rust-lang.org/cargo/reference/credential-provider-protocol.html) to authenticate with private Artifactory crate registries.

It uses the [JFrog CLI](https://jfrog.com/getcli/) to generate an access token.

It runs `jf atc --expiry 900` whenever cargo requests a token for crate registry whose hostname ends in ".jfrog.io".
It requires the user having run `jf login` already, and shouldn't be used if multiple Artifactory crate registries are used which require separate tokens.

TODO:
- [] configuration of expiry and index urls
- [] GitHub OIDC support
