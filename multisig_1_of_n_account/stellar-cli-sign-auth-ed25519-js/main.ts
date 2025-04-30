// CLI.
import { program } from "commander";
program
  .option("--secret-key [SECRET_KEY]", "", decodeHex)
  .option(
    "--network-passphrase <network-passphrase>",
    "",
    "Test SDF Network ; September 2015",
  )
  .option(
    "--signature-expiration-ledger <signature-expiration-ledger>",
    "",
    Number,
  );
program.parse();
const opts = program.opts();

// Collect stdin.
let stdin = "";
const decoder = new TextDecoder();
for await (const chunk of Deno.stdin.readable) {
  stdin += decoder.decode(chunk);
}

// Write to stderr, for human logs.
function stderr(...args: string[]) {
  const s = args.join(" ") + "\n";
  const encoder = new TextEncoder();
  const data = encoder.encode(s);
  Deno.stderr.writeSync(data);
}

// Write to stdout, for outputting the transaction envelope.
function stdout(...args: string[]) {
  const s = args.join(" ");
  const encoder = new TextEncoder();
  const data = encoder.encode(s);
  Deno.stdout.writeSync(data);
}

import { decodeHex, encodeHex } from "jsr:@std/encoding/hex";
import { hash, Keypair, xdr } from "@stellar/stellar-sdk";
import init, { decode } from "@stellar/stellar-xdr-json";
await init();

// Derive public key from secret key, and prepare keypair for signing.
const keypair = Keypair.fromRawEd25519Seed(opts.secretKey);
stderr("PublicKey:", encodeHex(keypair.rawPublicKey()));

// Read in transaction envelope from stdin.
const txe = xdr.TransactionEnvelope.fromXDR(stdin, "base64");

// Iterate over the auths that are needed for signing and sign each.
if (txe.switch() != xdr.EnvelopeType.envelopeTypeTx()) {
  stderr("transaction envelope type unsupported");
  Deno.exit(1);
}
for (const op of txe.v1().tx().operations()) {
  if (op.body().switch() != xdr.OperationType.invokeHostFunction()) {
    continue;
  }
  for (const auth of op.body().invokeHostFunctionOp().auth()) {
    if (
      auth.credentials().switch() !=
        xdr.SorobanCredentialsType.sorobanCredentialsAddress()
    ) {
      continue;
    }
    const creds = auth.credentials().address();
    stderr("Authorizing:");
    stderr(decode("SorobanAuthorizationEntry", auth.toXDR("base64")));

    // Build the payload that the network will expect to be signed for the authorization.
    const payload = xdr.HashIdPreimage.envelopeTypeSorobanAuthorization(
      new xdr.HashIdPreimageSorobanAuthorization({
        networkId: hash(new TextEncoder().encode(opts.networkPassphrase)),
        nonce: auth.credentials().address().nonce(),
        signatureExpirationLedger: opts.signatureExpirationLedger,
        invocation: auth.rootInvocation(),
      }),
    ).toXDR();
    const payload_hash = hash(payload);
    stderr(`Payload: ${encodeHex(payload_hash)}`);

    // Sign the payload hash.
    const signature = keypair.sign(payload_hash);

    // Modify the credentials on the auth to contain the signature.
    creds.signatureExpirationLedger(opts.signatureExpirationLedger);
    creds.signature(
      xdr.ScVal.scvMap([
        new xdr.ScMapEntry(
          {
            key: xdr.ScVal.scvSymbol("public_key"),
            val: xdr.ScVal.scvBytes(keypair.rawPublicKey()),
          },
        ),
        new xdr.ScMapEntry(
          {
            key: xdr.ScVal.scvSymbol("signature"),
            val: xdr.ScVal.scvBytes(signature),
          },
        ),
      ]),
    );
    stderr("Authorized:");
    stderr(decode("SorobanAuthorizationEntry", auth.toXDR("base64")));
  }
}

// Output the modified transaction to stdout so that it can be piped to simulation and sending.
stdout(txe.toXDR("base64"));
