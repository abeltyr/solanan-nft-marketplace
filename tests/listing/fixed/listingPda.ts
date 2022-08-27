import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token";
import { clusterApiUrl, Connection } from "@solana/web3.js";
import { assert, expect } from "chai";
import { Listings } from "../../../target/types/listings";

describe("listings", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let payer: anchor.web3.Keypair;
  let buyer: anchor.web3.Keypair;
  let mint: anchor.web3.PublicKey;
  let connection: anchor.web3.Connection;
  let ownerTokenAddress: anchor.web3.PublicKey;
  let buyerTokenAddress: anchor.web3.PublicKey;
  let nftPda;
  let listingPda;
  let programAccount: anchor.web3.Keypair;
  const program = anchor.workspace.Listings as Program<Listings>;

  it("setup data", async () => {
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("../../keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let programAccountKey = require("../../../target/deploy/listings-keypair.json");
    const programSecretKey = Uint8Array.from(programAccountKey);
    programAccount = anchor.web3.Keypair.fromSecretKey(programSecretKey);

    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    mint = new anchor.web3.PublicKey(
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
  });
  it("Create Nft Pda ", async () => {
    console.log(
      "Create Nft Pda --------------------------------------------------------------------",
    );

    let findNftPda = await anchor.web3.PublicKey.findProgramAddress(
      [mint.toBuffer(), Buffer.from("_nft_listing_data")],
      program.programId,
    );

    nftPda = findNftPda[0];
    console.log(await program.account.nftListingData.fetch(nftPda));
    const nftListingData = await program.account.nftListingData.fetch(nftPda);
    let count = nftListingData.amount;

    console.log("nft listing count", count);
    let listing = await anchor.web3.PublicKey.findProgramAddress(
      [
        nftPda.toBuffer(),
        Buffer.from("_Fixed_Price_"),
        Buffer.from(`${count}`),
      ],
      program.programId,
    );

    listingPda = listing[0];
    console.log(listingPda.toString());
    // create the nft listing
  });
  it("Fail Create Listing Pda by owner issue", async () => {
    console.log(
      "Fail Creating a second Listing Pda with invalid Data --------------------------------------------------------------------",
    );

    try {
      let transaction = await program.methods
        .createFixedPriceListingPda()
        .accounts({
          seller: buyer.publicKey,
          sellerToken: buyerTokenAddress,
          nftListingAccount: nftPda,
          listingAccount: listingPda,
        })
        .signers([buyer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.logs);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing Pda by the token account miss match", async () => {
    console.log(
      "Fail Creating a second Listing Pda with invalid Data --------------------------------------------------------------------",
    );

    try {
      let transaction = await program.methods
        .createFixedPriceListingPda()
        .accounts({
          seller: buyer.publicKey,
          sellerToken: ownerTokenAddress,
          nftListingAccount: nftPda,
          listingAccount: listingPda,
        })
        .signers([buyer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.logs);
      assert.isNotNull(e);
    }
  });
  it("Create Listing Pda", async () => {
    console.log(
      "Create Listing Pda --------------------------------------------------------------------",
    );

    try {
      let transaction = await program.methods
        .createFixedPriceListingPda()
        .accounts({
          seller: payer.publicKey,
          sellerToken: ownerTokenAddress,
          nftListingAccount: nftPda,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      console.log("fixed price listing pda transaction signature", transaction);
      const listingData = await program.account.fixedPriceListingData.fetch(
        listingPda,
      );
      console.log("listingData", listingData);
    } catch (e) {
      console.log("Listing Pda Exist", e.logs);
    }
  });
});
