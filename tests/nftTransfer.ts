import * as anchor from "@project-serum/anchor";
import { clusterApiUrl, Connection } from "@solana/web3.js";
import {
  getOrCreateAssociatedTokenAccount,
  transfer,
  revoke,
} from "@solana/spl-token";
describe("Mint the Nft", async () => {
  let connection;
  let payer;
  let buyer;
  let ownerTokenAddress: anchor.web3.PublicKey;
  let buyerTokenAddress: anchor.web3.PublicKey;

  it("Setup An account", async () => {
    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("./keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    const mint = new anchor.web3.PublicKey(
      "Gcfx17cZF6pniqKPzfwXtdHShZvZ9urnTQ6sMJ8Yx9Hm",
    );

    const payerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      payer.publicKey,
    );
    ownerTokenAddress = payerTokenAccount.address;
    const buyerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      buyer,
      mint,
      buyer.publicKey,
    );
    buyerTokenAddress = buyerTokenAccount.address;
    // await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
  });

  it.skip("Transfer Nft!", async () => {
    const signature = await transfer(
      connection,
      payer,
      ownerTokenAddress,
      buyerTokenAddress,
      payer, // or pass fromPublicKey
      1, // tokens have 6 decimals of precision so your amount needs to have the same
    );

    console.log("signature", signature);
  });
  it("revoke Nft!", async () => {
    const signature = await revoke(connection, payer, ownerTokenAddress, payer);

    console.log("signature", signature);
  });
});
