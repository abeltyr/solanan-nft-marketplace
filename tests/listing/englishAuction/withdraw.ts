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
  let nftPda;
  let listingPda;
  let bidPda;
  let programAccount: anchor.web3.Keypair;
  const program = anchor.workspace.Listings as Program<Listings>;

  it("setup data", async () => {
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("../../keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let accountK1ey = require("../../keypairs/first.json");
    const secret1Key = Uint8Array.from(accountK1ey);
    buyer1 = anchor.web3.Keypair.fromSecretKey(secret1Key);

    let programAccountKey = require("../../../target/deploy/listings-keypair.json");
    const programSecretKey = Uint8Array.from(programAccountKey);
    programAccount = anchor.web3.Keypair.fromSecretKey(programSecretKey);

    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    mint = new anchor.web3.PublicKey(
      "xH36fd7YokUVCCiYL2xnHzmz6uaQXnbETETt8tbxcpU",
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
  });
  it("check buyer balance", async () => {
    console.log(
      "check For bid --------------------------------------------------------------------",
    );

    const data = await connection.getAccountInfo(bidPda);
    console.log("data", data);
    const payerData = await connection.getAccountInfo(payer.publicKey);
    console.log("payerData", payerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
  });
  it("withdraw ", async () => {
    try {
      console.log(
        "withdraw fund fom bid --------------------------------------------------------------------",
      );

      let transaction = await program.methods
        .withdrawBidEnglishAuction()
        .accounts({
          auctionAccount: listingPda,
          bidAccount: bidPda,
          withdrawer: payer.publicKey,
          bidAccountVault: bidPda,
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
  it("check buyer balance", async () => {
    console.log(
      "check For bid --------------------------------------------------------------------",
    );

    const data = await connection.getAccountInfo(bidPda);
    console.log("data", data);
    const payerData = await connection.getAccountInfo(payer.publicKey);
    console.log("payerData", payerData.lamports / anchor.web3.LAMPORTS_PER_SOL);
  });
});
