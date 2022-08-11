import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Listings } from "../target/types/listings";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
  createSetAuthorityInstruction,
  AuthorityType,
  approveChecked,
  transfer,
} from "@solana/spl-token";
import { clusterApiUrl, Connection } from "@solana/web3.js";

describe("listings", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let payer: anchor.web3.Keypair;
  let buyer: anchor.web3.Keypair;
  let programAccount: anchor.web3.Keypair;
  const program = anchor.workspace.Listings as Program<Listings>;

  it("setup users", async () => {
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("./keypairs/first.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    let programAccountKey = require("../target/deploy/listings-keypair.json");
    const programSecretKey = Uint8Array.from(programAccountKey);
    programAccount = anchor.web3.Keypair.fromSecretKey(programSecretKey);
  });
  it("Listing", async () => {
    const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
      "85ydm3AHhcL5CP5ErUmHyWcueAMbe6Wqyb4ETbYJZMis",
    );

    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
      [mint.toBuffer(), Buffer.from("_state")],
      program.programId,
    );

    // create the nft listing
    try {
      let transaction = await program.methods
        .createNftListing()
        .accounts({
          mint: mint,
          owner: payer.publicKey,
          nftListingAccount: pda,
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", transaction);
    } catch (e) {}

    const ownerTokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer.publicKey,
    });
    let startTime: number = new Date().getTime() / 1000 + 30000;
    let endTime: number = new Date().getTime() / 1000 + 6000 * 6000 * 24;
    let transaction = await program.methods
      .fixedPriceListing(new anchor.BN(startTime), new anchor.BN(endTime))
      .accounts({
        owner: payer.publicKey,
        nftListingAccount: pda,
        programAccount: programAccount.publicKey,
        ownerTokenAccount: ownerTokenAddress,
      })
      .signers([payer, programAccount])
      .rpc();
    console.log("Your transaction signature", transaction);
  });
});
