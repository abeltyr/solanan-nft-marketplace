import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token";
import { clusterApiUrl, Connection } from "@solana/web3.js";
import { Listings } from "../../target/types/listings";

describe("english", () => {
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

    let accountKey = require("./keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let programAccountKey = require("../target/deploy/listings-keypair.json");
    const programSecretKey = Uint8Array.from(programAccountKey);
    programAccount = anchor.web3.Keypair.fromSecretKey(programSecretKey);

    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    mint = new anchor.web3.PublicKey(
      "9cmjNxLQJzJpRYE3gkzoJ7Bk1A5aiPxXhsABz2529Ryr",
    );

    ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer.publicKey,
    });
  });
  it("Create Nft Pda ", async () => {
    console.log(
      "Create Nft Pda --------------------------------------------------------------------",
    );

    let findNftPda = await anchor.web3.PublicKey.findProgramAddress(
      [mint.toBuffer(), Buffer.from("_state")],
      program.programId,
    );

    nftPda = findNftPda[0];
    // create the nft listing
    try {
      let transaction = await program.methods
        .createNftListingPda()
        .accounts({
          mint: mint,
          owner: payer.publicKey,
          nftListingAccount: nftPda,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", transaction);
    } catch (e) {
      console.log(
        "Nft Pda Exist",
        // e
      );
    }
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
        Buffer.from("_English_Auction_"),
        Buffer.from(`${count}`),
      ],
      program.programId,
    );

    listingPda = listing[0];

    try {
      let transaction = await program.methods
        .createEnglishAuctionListingPda(`${count}`)
        .accounts({
          mint: mint,
          seller: payer.publicKey,
          nftListingAccount: nftPda,
          listingAccount: listingPda,
        })
        .signers([payer])
        .rpc();

      console.log("fixed price listing pda transaction signature", transaction);
      const listingData = await program.account.englishAuctionListingData.fetch(
        listing[0],
      );
      console.log("listingData", listingData);
    } catch (e) {
      console.log("Listing Pda Exist", e);
    }
  });
  it("Create English Auction Listing", async () => {
    console.log(
      "Create Listing --------------------------------------------------------------------",
    );
    const startTime: number = new Date().getTime() / 1000 + 30000;
    const endTime: number = new Date().getTime() / 1000 + 6000 * 6000 * 24;

    const saleAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL;
    try {
      let transaction = await program.methods
        .createEnglishAuctionListing(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          new anchor.BN(saleAmount),
        )
        .accounts({
          owner: payer.publicKey,
          nftListingAccount: nftPda,
          programAccount: programAccount.publicKey,
          ownerTokenAccount: ownerTokenAddress,
          listingAccount: listingPda,
        })
        .signers([payer, programAccount])
        .rpc();
      console.log("Your transaction signature", transaction);
      const listingData = await program.account.englishAuctionListingData.fetch(
        listingPda,
      );
      console.log("listingData", listingData);
    } catch (e) {
      console.log("error", e);
      // throw new Error(e);
    }
  });
  it("bid ", async () => {
    try {
      console.log(
        "Bid For Listing --------------------------------------------------------------------",
      );

      const buyerData = await connection.getAccountInfo(buyer.publicKey);
      console.log(
        "buyerData",
        buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL,
      );
      const buyerTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        buyer,
        mint,
        buyer.publicKey,
      );
      buyerTokenAddress = buyerTokenAccount.address;

      // buyerTokenAddress = await anchor.utils.token.associatedAddress({
      //   mint: mint,
      //   owner: buyer.publicKey,

      // });
      let transaction = await program.methods
        .buyNftFixedPriceListing()
        .accounts({
          nftListingAccount: nftPda,
          listingAccount: listingPda,
          mint: mint,
          buyer: buyer.publicKey,
          seller: payer.publicKey,
          buyerTokenAccount: buyerTokenAddress,
          sellerTokenAccount: ownerTokenAddress,
          programAccount: programAccount.publicKey,
        })
        .signers([buyer, programAccount])
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
  it("close ", async () => {
    try {
      console.log(
        "Close Nft Listing --------------------------------------------------------------------",
      );
      let transaction = await program.methods
        .closeEnglishAuctionListing()
        .accounts({
          nftListingAccount: nftPda,
          listingAccount: listingPda,
          mint: mint,
          owner: payer.publicKey,
          ownerTokenAccount: ownerTokenAddress,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", transaction);
      const listingData = await program.account.englishAuctionListingData.fetch(
        listingPda,
      );
      const nftData = await program.account.nftListingData.fetch(nftPda);
      console.log({ listingData, nftData });
    } catch (e) {
      throw new Error(e);
      console.log("error", e);
    }
  });
});
