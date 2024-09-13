#!/usr/bin/env -S deno run -A

import StellarSdk, {
  Address,
  Keypair,
  Networks,
  Operation,
  StrKey,
  TransactionBuilder,
  xdr,
} from "@stellar/stellar-sdk";
import * as hex from "@std/encoding/hex";

const SERVER = "https://soroban-testnet.stellar.org";
const NETWORK = Networks.TESTNET;

const wasm = Deno.readFileSync(
  "target/wasm32-unknown-unknown/release/soroban_hello_world_contract.wasm",
);

const wasmHash = new Uint8Array(await crypto.subtle.digest("SHA-256", wasm));
console.log("Wasm:", hex.encodeHex(wasmHash));

const key = Keypair.random();
console.log("Key:", key.publicKey());
const server = new StellarSdk.rpc.Server(SERVER);

console.log("Funding test account...");
const _ = await server.requestAirdrop(key.publicKey());
const account = await server.getAccount(key.publicKey());

{
  console.log("Upload wasm...");
  const tx = await server.prepareTransaction(
    new TransactionBuilder(account, {
      fee: "100",
    })
      .setNetworkPassphrase(NETWORK)
      .setTimeout(30)
      .addOperation(
        Operation.uploadContractWasm({
          wasm,
        }),
      )
      .build(),
  );
  tx.sign(key);
  const txHash = hex.encodeHex(tx.hash());
  console.log("Tx hash:", txHash);
  const sendResp = await server.sendTransaction(tx);
  console.log(sendResp.status);
  if (sendResp.status !== "PENDING") {
    throw "";
  }
  let getResp;
  do {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    console.log(".");
    getResp = await server.getTransaction(txHash);
  } while (getResp.status === "NOT_FOUND");
  console.log(getResp.status);
}

{
  console.log("Deploy contract...");
  const salt = Keypair.random().xdrPublicKey().value();
  const contractAddr = StrKey.encodeContract(
    await crypto.subtle.digest(
      "SHA-256",
      xdr.HashIdPreimage.envelopeTypeContractId(
        new xdr.HashIdPreimageContractId({
          networkId: new Uint8Array(
            await crypto.subtle.digest(
              "SHA-256",
              new TextEncoder().encode(NETWORK),
            ),
          ),
          contractIdPreimage: xdr.ContractIdPreimage
            .contractIdPreimageFromAddress(
              new xdr.ContractIdPreimageFromAddress({
                address: Address.fromString(key.publicKey()).toScAddress(),
                salt,
              }),
            ),
        }),
      ).toXDR(),
    ),
  );
  console.log("Contract address:", contractAddr);
  const tx = await server.prepareTransaction(
    new TransactionBuilder(account, {
      fee: "100",
    })
      .setNetworkPassphrase(NETWORK)
      .setTimeout(30)
      .addOperation(
        Operation.createCustomContract({
          address: Address.fromString(key.publicKey()),
          wasmHash,
          salt,
        }),
      )
      .build(),
  );
  tx.sign(key);
  const tx_hash = hex.encodeHex(tx.hash());
  console.log("Tx hash:", tx_hash);
  const send_resp = await server.sendTransaction(tx);
  console.log(send_resp.status);
  if (send_resp.status !== "PENDING") {
    throw "";
  }
  let get_resp;
  do {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    console.log(".");
    get_resp = await server.getTransaction(tx_hash);
  } while (get_resp.status === "NOT_FOUND");
  console.log(get_resp.status);
}
