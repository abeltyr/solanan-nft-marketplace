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
  let bidPda;
  let programAccount: anchor.web3.Keypair;
  const program = anchor.workspace.Listings as Program<Listings>;

  it("setup data", async () => {
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("../keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let programAccountKey = require("../../target/deploy/listings-keypair.json");
    const programSecretKey = Uint8Array.from(programAccountKey);
    programAccount = anchor.web3.Keypair.fromSecretKey(programSecretKey);

    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    mint = new anchor.web3.PublicKey(
      "7EGHMZJ3iXJSRcpZvjEYLYMkSZPP2cjoFxXHMXpxRb5",
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
    let count = "20";
    // nftListingData.amount;

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
          sellerToken: ownerTokenAddress,
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
    const startTime: number = new Date().getTime() / 1000 + 5;
    const endTime: number = new Date().getTime() / 1000 + 60 * 60 * 24;

    console.log(startTime, endTime);
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
  it("bid pda", async () => {
    try {
      console.log(
        "Bid Pda For Listing --------------------------------------------------------------------",
      );

      let bidListingPda = await anchor.web3.PublicKey.findProgramAddress(
        [listingPda.toBuffer(), buyer.publicKey.toBuffer()],
        program.programId,
      );

      bidPda = bidListingPda[0];
      let transaction = await program.methods
        .createEnglishAuctionBidPda()
        .accounts({
          auctionAccount: listingPda,
          bidder: buyer.publicKey,
          mint: mint,
          bidAccount: bidPda,
        })
        .signers([buyer])
        .rpc();
      console.log("Your transaction signature", transaction);
    } catch (e) {
      console.log("error", e);
    }
  });
  it("check buyer balance", async () => {
    console.log(
      "check For Listing --------------------------------------------------------------------",
    );

    const buyerData = await connection.getAccountInfo(buyer.publicKey);
    console.log("buyerData", buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
  });
  it("bid ", async () => {
    try {
      console.log(
        "Bid For Listing --------------------------------------------------------------------",
      );

      const saleAmount = 0.016 * anchor.web3.LAMPORTS_PER_SOL;

      let transaction = await program.methods
        .bidEnglishAuction(new anchor.BN(saleAmount))
        .accounts({
          auctionAccount: listingPda,
          bidder: buyer.publicKey,
          mint: mint,
          bidAccount: bidPda,
          bidderTokenAccount: buyerTokenAddress,
          bidAccountVault: bidPda,
        })
        .signers([buyer])
        .rpc();
      console.log("Your transaction signature", transaction);
      const bidData = await program.account.englishAuctionListingBidData.fetch(
        bidPda,
      );
      const listingData = await program.account.englishAuctionListingData.fetch(
        listingPda,
      );

      console.log({ bidData, listingData });
    } catch (e) {
      console.log("error", e);
    }
  });
  it("check buyer balance", async () => {
    console.log(
      "check For Listing --------------------------------------------------------------------",
    );

    const buyerData = await connection.getAccountInfo(buyer.publicKey);
    console.log("buyerData", buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
  });
  it("close ", async () => {
    try {
      console.log(
        "Close Nft Listing --------------------------------------------------------------------",
      );
      console.log({
        nftListingAccount: nftPda,
        listingAccount: listingPda,
        mint: mint,
        owner: payer.publicKey,
        ownerTokenAccount: ownerTokenAddress,
      });

      let listingData = await program.account.englishAuctionListingData.fetch(
        listingPda,
      );
      let nftData = await program.account.nftListingData.fetch(nftPda);
      console.log({ listingData, nftData });
      let transaction = await program.methods
        .closeEnglishAuctionListing()
        .accounts({
          nftListingAccount: nftPda,
          listingAccount: listingPda,
          mint: mint,
          owner: payer.publicKey,
          ownerTokenAccount: ownerTokenAddress,
        })
        .signers([])
        .rpc();
      console.log("Your transaction signature", transaction);
      listingData = await program.account.englishAuctionListingData.fetch(
        listingPda,
      );
      nftData = await program.account.nftListingData.fetch(nftPda);
      console.log({ listingData, nftData });
    } catch (e) {
      console.log("error", e);
    }
  });
});
