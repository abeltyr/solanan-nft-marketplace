import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token";
import { clusterApiUrl, Connection } from "@solana/web3.js";
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
      "F5PBa9pqwUsVUYSffyADypu56FBPoV4LfGv8qbJHva6Z",
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
    console.log("nftPda", nftPda.toString());
    // create the nft listing
  });
  it("Create Listing Pda", async () => {
    console.log(
      "Create Listing Pda --------------------------------------------------------------------",
    );
    const nftListingData = await program.account.nftListingData.fetch(nftPda);
    let count = nftListingData.amount - 1;

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
    console.log(
      "listingPda",
      await program.account.fixedPriceListingData.fetch(listingPda),
    );
  });
  it("check buyer balance", async () => {
    console.log(
      "check For amount --------------------------------------------------------------------",
    );

    const buyerData = await connection.getAccountInfo(buyer.publicKey);
    console.log("buyerData", buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
    const payerData = await connection.getAccountInfo(payer.publicKey);
    console.log("payerData", payerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
  });
  it("buy ", async () => {
    try {
      console.log(
        "Buy Nft From Listing --------------------------------------------------------------------",
      );

      const buyerData = await connection.getAccountInfo(buyer.publicKey);
      console.log(
        "buyerData",
        buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL,
      );

      let transaction = await program.methods
        .buyNftFixedPriceListing()
        .accounts({
          nftListingAccount: nftPda,
          listingAccount: listingPda,
          buyer: buyer.publicKey,
          seller: payer.publicKey,
          buyerToken: buyerTokenAddress,
          sellerToken: ownerTokenAddress,
        })
        .signers([buyer])
        .rpc();
      console.log("Your transaction signature", transaction);
      const listingData = await program.account.fixedPriceListingData.fetch(
        listingPda,
      );
      const nftData = await program.account.nftListingData.fetch(nftPda);
      console.log({ listingData, nftData });
    } catch (e) {
      console.log("error", e);
      throw new Error(e);
    }
  });
  it("check buyer balance", async () => {
    console.log(
      "check For amount --------------------------------------------------------------------",
    );

    const buyerData = await connection.getAccountInfo(buyer.publicKey);
    console.log("buyerData", buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
    const payerData = await connection.getAccountInfo(payer.publicKey);
    console.log("payerData", payerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
  });
});
