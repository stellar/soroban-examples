import { program } from "commander";
import { stderr, stdin, stdout } from "./io.ts";
import { decodeHex, encodeHex } from "@std/encoding/hex";
import { hash, Keypair, xdr } from "@stellar/stellar-sdk";
import init, { decode } from "@stellar/stellar-xdr-json";
await init();

// CLI.
program
  .configureOutput({
    getOutHasColors: () => false,
    getErrHasColors: () => false,
  })
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

// Derive the network id from the network passphrase.
const networkId = hash(new TextEncoder().encode(opts.networkPassphrase));

// Derive public key from secret key, and prepare keypair for signing.
const keypair = Keypair.fromRawEd25519Seed(opts.secretKey);
stderr("PublicKey:", encodeHex(keypair.rawPublicKey()));

// Read in transaction envelope from stdin.
const txe = xdr.TransactionEnvelope.fromXDR(await stdin(), "base64");
if (txe.switch() != xdr.EnvelopeType.envelopeTypeTx()) {
  stderr("transaction envelope type unsupported");
  Deno.exit(1);
}

// Extract mutable references to the parts of the auths that are needed for signing.
const auths = txe.v1().tx().operations()
  .map((op) => op.body())
  .filter((op) => op.switch() == xdr.OperationType.invokeHostFunction())
  .map((op) =>
    op.invokeHostFunctionOp().auth()
      .filter((auth) =>
        auth.credentials().switch() ==
          xdr.SorobanCredentialsType.sorobanCredentialsAddress()
      )
      .map((auth) =>
        <[xdr.SorobanAuthorizedInvocation, xdr.SorobanAddressCredentials]> [
          auth.rootInvocation(),
          auth.credentials().address(),
        ]
      )
  )
  .flat();

// Sign each auth.
// TODO:It would be wise to only sign auths matching the contract address that are intended to
// sign for, or to ask the user to confirm each auth.
for (const [invocation, creds] of auths) {
  stderr("Authorizing:");
  stderr(decode("SorobanAuthorizedInvocation", invocation.toXDR("base64")));
  stderr(decode("SorobanAddressCredentials", creds.toXDR("base64")));

  // Build the payload that the network will expect to be signed for the authorization.
  const payload = xdr.HashIdPreimage.envelopeTypeSorobanAuthorization(
    new xdr.HashIdPreimageSorobanAuthorization({
      networkId,
      nonce: creds.nonce(),
      signatureExpirationLedger: opts.signatureExpirationLedger,
      invocation,
    }),
  ).toXDR();
  const payload_hash = hash(payload);
  stderr("Payload:", encodeHex(payload_hash));

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
  stderr(decode("SorobanAddressCredentials", creds.toXDR("base64")));
}

// Output the modified transaction to stdout so that it can be piped to simulation and sending.
stdout(txe.toXDR("base64"));
