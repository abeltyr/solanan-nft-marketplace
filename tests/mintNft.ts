import {
  findMasterEditionV2Pda,
  findMetadataPda,
} from "@metaplex-foundation/js";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token";
import {
  clusterApiUrl,
  Connection,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import { assert } from "chai";
import { Listings } from "../target/types/listings";
import * as mpl from "@metaplex-foundation/mpl-token-metadata";

describe("listings", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let payer: anchor.web3.Keypair;
  let buyer: anchor.web3.Keypair;
  let mint: anchor.web3.Keypair;
  let connection: anchor.web3.Connection;
  let tokenAddress: anchor.web3.PublicKey;
  let nftOwnerPda;
  const program = anchor.workspace.Listings as Program<Listings>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
  );

  it("setup data", async () => {
    let payerAccountKey = require(process.env.ANCHOR_WALLET);
    const payerSecretKey = Uint8Array.from(payerAccountKey);
    payer = anchor.web3.Keypair.fromSecretKey(payerSecretKey);

    let accountKey = require("./keypairs/second.json");
    const secretKey = Uint8Array.from(accountKey);
    buyer = anchor.web3.Keypair.fromSecretKey(secretKey);

    connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  });
  it("mint Nft Pda ", async () => {
    mint = anchor.web3.Keypair.generate();
    console.log(`mint: ${mint.publicKey}`);

    let findNftOwnerPda = await anchor.web3.PublicKey.findProgramAddress(
      [mint.publicKey.toBuffer(), Buffer.from("_authority_")],
      program.programId,
    );

    nftOwnerPda = findNftOwnerPda[0];
    console.log(`nftOwnerPda: ${nftOwnerPda.toString()}`);

    let findNftPda = await anchor.web3.PublicKey.findProgramAddress(
      [mint.publicKey.toBuffer(), Buffer.from("_nft_listing_data")],
      program.programId,
    );

    const nftPda = findNftPda[0];
    console.log(`nftPda: ${nftPda.toString()}`);

    tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: payer.publicKey,
    });

    console.log(`tokenAddress: ${tokenAddress}`);

    try {
      await program.methods
        .mintNft()
        .accounts({
          mint: mint.publicKey,
          tokenAccount: tokenAddress,
          mintAuthority: payer.publicKey,
          nftAuthorityAccount: nftOwnerPda,
          nftListingAccount: nftPda,
        })
        .signers([mint])
        .rpc();
      console.log(await program.account.nftListingData.fetch(nftPda));
    } catch (e) {
      console.log(e);
    }
  });
  it("setup Nft metadata Pda ", async () => {
    const metadataPDA = await findMetadataPda(mint.publicKey);

    console.log("Metadata initialized", metadataPDA.toString());

    const masterEditionPda = await findMasterEditionV2Pda(mint.publicKey);

    console.log(
      "Master edition metadata initialized",
      masterEditionPda.toString(),
    );

    try {
      await program.methods
        .setupNftMetadata("MeMusic", "MM", "")
        .accounts({
          masterEdition: masterEditionPda,
          metadata: metadataPDA,
          mint: mint.publicKey,
          tokenAccount: tokenAddress,
          mintAuthority: payer.publicKey,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          nftAuthorityAccount: nftOwnerPda,
        })
        .signers([mint])
        .rpc();
    } catch (e) {
      console.log(e);
    }
  });
});
