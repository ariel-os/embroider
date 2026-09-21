//! embroider: CLI front-end for the `embroider` library, signing a `SUIT_Envelope`'s
//! authentication wrapper with ECDSA (ES256/ES384).

use std::{fs, process::ExitCode};

use embroider::{Algorithm, Error, Signer};

const USAGE: &str = "usage: embroider --input <envelope.cbor> --output <signed.cbor> \
--alg <es256|es384> (--key <key.pem> | --key-hex <hex-scalar>)";

/// Parsed CLI arguments for a single signing invocation.
struct Args {
    input: String,
    output: String,
    algorithm: Algorithm,
    key_pem_path: Option<String>,
    key_hex: Option<String>,
}

/// Parses `std::env::args()` into [`Args`], returning a human-readable error on bad usage.
fn parse_args() -> Result<Args, String> {
    let mut input = None;
    let mut output = None;
    let mut algorithm = None;
    let mut key_pem_path = None;
    let mut key_hex = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = Some(args.next().ok_or("--input requires a value")?),
            "--output" => output = Some(args.next().ok_or("--output requires a value")?),
            "--alg" => {
                let value = args.next().ok_or("--alg requires a value")?;
                algorithm = Some(match value.as_str() {
                    "es256" => Algorithm::Es256,
                    "es384" => Algorithm::Es384,
                    other => {
                        return Err(format!("unknown --alg value: {other} (expected es256 or es384)"))
                    }
                });
            }
            "--key" => key_pem_path = Some(args.next().ok_or("--key requires a value")?),
            "--key-hex" => key_hex = Some(args.next().ok_or("--key-hex requires a value")?),
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Args {
        input: input.ok_or("missing required --input <path>")?,
        output: output.ok_or("missing required --output <path>")?,
        algorithm: algorithm.ok_or("missing required --alg <es256|es384>")?,
        key_pem_path,
        key_hex,
    })
}

/// Loads the `Signer` requested by `args`, from either a PEM file or a raw hex scalar.
fn load_signer(args: &Args) -> Result<Signer, Error> {
    match (&args.key_pem_path, &args.key_hex) {
        (Some(path), None) => {
            let pem = fs::read_to_string(path)?;
            Signer::from_pem(&pem, args.algorithm)
        }
        (None, Some(hex_str)) => {
            let bytes = hex::decode(hex_str)
                .map_err(|e| Error::InvalidKey(format!("invalid --key-hex value: {e}")))?;
            Signer::from_raw_bytes(&bytes, args.algorithm)
        }
        (Some(_), Some(_)) => Err(Error::InvalidKey("specify only one of --key or --key-hex".into())),
        (None, None) => Err(Error::InvalidKey(
            "missing required --key <path> or --key-hex <hex>".into(),
        )),
    }
}

/// Loads the key and input envelope, signs it, and writes the result to `args.output`.
fn run(args: &Args) -> Result<(), Error> {
    let signer = load_signer(args)?;
    let envelope_bytes = fs::read(&args.input)?;
    let signed_envelope = embroider::sign_envelope(&envelope_bytes, &signer)?;
    fs::write(&args.output, signed_envelope)?;
    Ok(())
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("error: {msg}\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
