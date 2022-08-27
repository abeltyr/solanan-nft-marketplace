import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token";
import { clusterApiUrl, Connection } from "@solana/web3.js";
import { Listings } from "../../../target/types/listings";

describe("english auction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let payer: anchor.web3.Keypair;
  let buyer: anchor.web3.Keypair;
  let buyer1: anchor.web3.Keypair;
  let mint: anchor.web3.PublicKey;
  let connection: anchor.web3.Connection;
  let ownerTokenAddress: anchor.web3.PublicKey;
  let buyerTokenAddress: anchor.web3.PublicKey;
  let buyer1TokenAddress: anchor.web3.PublicKey;
  let nftPda;
  let listingPda;
  let bidPda;
  let bidPda1;
  let programAccount: anchor.web3.Keypair;
  const program = anchor.workspace.Listings as Program<Listings>;

  it("setup data", async () => {
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("../../keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let accountKey1 = require("../../keypairs/first.json");
    const secretKey1 = Uint8Array.from(accountKey1);
    buyer1 = anchor.web3.Keypair.fromSecretKey(secretKey1);

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
    const buyer1TokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      buyer1,
      mint,
      buyer1.publicKey,
    );
    buyer1TokenAddress = buyer1TokenAccount.address;
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
        Buffer.from("_English_Auction_"),
        Buffer.from(`${count}`),
      ],
      program.programId,
    );

    listingPda = listing[0];
  });

  it("bid pda", async () => {
    console.log(
      "Bid Pda For Listing --------------------------------------------------------------------",
    );

    let bidListingPda = await anchor.web3.PublicKey.findProgramAddress(
      [listingPda.toBuffer(), buyer.publicKey.toBuffer()],
      program.programId,
    );

    bidPda = bidListingPda[0];

    let bidListingPda1 = await anchor.web3.PublicKey.findProgramAddress(
      [listingPda.toBuffer(), buyer1.publicKey.toBuffer()],
      program.programId,
    );

    bidPda1 = bidListingPda1[0];
  });

  it("check buyer balance", async () => {
    console.log(
      "check For bid --------------------------------------------------------------------",
    );

    const data = await connection.getAccountInfo(bidPda);
    console.log("data", data);
    const bidPdaData = await program.account.englishAuctionListingBidData.fetch(
      bidPda,
    );
    const bidPdaData1 =
      await program.account.englishAuctionListingBidData.fetch(bidPda1);
    console.log({ bidPdaData, bidPdaData1 });
    const payerData = await connection.getAccountInfo(payer.publicKey);
    console.log("payerData", payerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
    const buyerData = await connection.getAccountInfo(buyer.publicKey);
    console.log("buyerData", buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
    const buyer1Data = await connection.getAccountInfo(buyer1.publicKey);
    console.log(
      "buyer1Data",
      buyer1Data.lamports / anchor.web3.LAMPORTS_PER_SOL,
    );
  });
  it("seller withdraw ", async () => {
    try {
      console.log(
        "withdraw fund fom bid --------------------------------------------------------------------",
      );

      let transaction = await program.methods
        .withdrawBidEnglishAuction()
        .accounts({
          listingAccount: listingPda,
          bidAccount: bidPda1,
          withdrawer: payer.publicKey,
          bidAccountVault: bidPda1,
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
      console.log("error", e);
    }
  });
  it("bidder withdraw ", async () => {
    try {
      console.log(
        "withdraw fund fom bid --------------------------------------------------------------------",
      );

      let transaction = await program.methods
        .withdrawBidEnglishAuction()
        .accounts({
          listingAccount: listingPda,
          bidAccount: bidPda,
          withdrawer: buyer.publicKey,
          bidAccountVault: bidPda,
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
      "check For bid --------------------------------------------------------------------",
    );

    const data = await connection.getAccountInfo(bidPda);
    console.log("data", data);
    const bidPdaData = await program.account.englishAuctionListingBidData.fetch(
      bidPda,
    );
    const bidPdaData1 =
      await program.account.englishAuctionListingBidData.fetch(bidPda1);
    console.log({ bidPdaData, bidPdaData1 });
    const payerData = await connection.getAccountInfo(payer.publicKey);
    console.log("payerData", payerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
    const buyerData = await connection.getAccountInfo(buyer.publicKey);
    console.log("buyerData", buyerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
    const buyer1Data = await connection.getAccountInfo(buyer1.publicKey);
    console.log(
      "buyer1Data",
      buyer1Data.lamports / anchor.web3.LAMPORTS_PER_SOL,
    );
  });
});
