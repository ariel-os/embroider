# embroider (SUIT Manifest Signer)

Signs a SUIT_Envelope CBOR file (as produced by
[taylor](https://github.com/schnitzm/taylor) or any spec-conformant SUIT encoder) by adding
a `COSE_Sign1` authentication block over its existing digest.

Supports ECDSA ES256 (NIST P-256) and ES384 (NIST P-384).

## Status

Implemented and tested — see [docs/PLAN.md](docs/PLAN.md) for the full design and
remaining follow-up ideas.

## Usage

```
cargo run -- --input envelope.cbor --output signed.cbor --alg es256 --key private-key.pem
```

`--alg` accepts `es256` or `es384`. The private key can be supplied either as a PEM file
via `--key <path>` (PKCS8 or SEC1) or as raw scalar bytes in hex via `--key-hex <hex>`.

## Testing

Run the full automated test suite (unit tests plus an end-to-end CLI integration test):

```
cargo test
```

To manually exercise the CLI against a real envelope:

```
# generate a throwaway P-256 test key
openssl ecparam -name prime256v1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt > test-key.pem

# sign an envelope file
cargo run -- --input path/to/envelope.cbor --output signed.cbor --alg es256 --key test-key.pem

# inspect the result
xxd signed.cbor | head
```

## Design references

- `docs/suit-manifest.cddl` / `docs/suit-manifest-extension.cddl`: the CDDL this tool's
  output must conform to (`SUIT_Authentication`, `SUIT_Digest`, `SUIT_Envelope`).
- [RFC 9052](https://www.rfc-editor.org/rfc/rfc9052) (COSE): `COSE_Sign1` structure and
  `Sig_structure` construction.

## Copyright & License

embroider is licensed under MIT license ([LICENSE](LICENSE))
