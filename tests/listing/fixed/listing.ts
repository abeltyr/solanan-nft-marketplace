import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { clusterApiUrl, Connection } from "@solana/web3.js";
import { assert } from "chai";
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
    console.log(process.env.ANCHOR_WALLET);
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("../../keypairs/first.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let programAccountKey = require("../../../target/deploy/listings-keypair.json");
    const programSecretKey = Uint8Array.from(programAccountKey);
    programAccount = anchor.web3.Keypair.fromSecretKey(programSecretKey);

    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    mint = new anchor.web3.PublicKey(
      "DYXESyv4NouW6j8fquDZZeuEW7ooymvQe5uKWMZPiLEm",
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
    console.log("nftPda", await program.account.nftListingData.fetch(nftPda));
    // create the nft listing
  });
  it("Create Listing Pda", async () => {
    console.log(
      "Create Listing Pda --------------------------------------------------------------------",
    );
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
  });
  it("Fail Create Listing by providing invalid nftPda", async () => {
    console.log(
      "Fail Create Listing by providing invalid nftPda --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 3;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const mint = new anchor.web3.PublicKey(
      "dEmsanDAqKSC4C9EFGx4JPgGMYk9Bs3f5oZKdTcMAyg",
    );

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;
    const findNftPda = await anchor.web3.PublicKey.findProgramAddress(
      [mint.toBuffer(), Buffer.from("_nft_listing_data")],
      program.programId,
    );

    const nftPda = findNftPda[0];

    console.log(listingPda.toString());
    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing by invalid owner", async () => {
    console.log(
      "Fail Create Listing by invalid owner --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 3;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;

    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: buyer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: buyerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([buyer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing by invalid token Address", async () => {
    console.log(
      "Fail Create Listing by invalid token Address --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 3;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;

    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: buyerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing by invalid start Date", async () => {
    console.log(
      "Fail Create Listing by invalid start Date --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 - 300;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;

    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing by invalid end Date", async () => {
    console.log(
      "Fail Create Listing by invalid end Date --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 300;
    const endTime: number = new Date().getTime() / 1000;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;

    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing by invalid price", async () => {
    console.log(
      "Fail Create Listing by invalid price --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 300;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const saleAmount = 0 * anchor.web3.LAMPORTS_PER_SOL;

    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Fail Create Listing by invalid owner and token address", async () => {
    console.log(
      "Fail Create Listing by invalid owner and token address--------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 3;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;

    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: buyer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([buyer])
        .rpc();

      assert.isNull(transaction);
    } catch (e) {
      console.log(e.error);
      assert.isNotNull(e);
    }
  });
  it("Create Listing", async () => {
    console.log(
      "Create Listing --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 3;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;
    try {
      let transaction = await program.methods
        .createFixedPriceListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          sellerToken: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", transaction);
      const listingData = await program.account.fixedPriceListingData.fetch(
        listingPda,
      );
      console.log("listingData", listingData);
    } catch (e) {
      console.log("error", e);
    }
  });
});
