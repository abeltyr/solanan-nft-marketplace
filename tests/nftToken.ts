import * as anchor from "@project-serum/anchor";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { Listings } from "../target/types/listings";
import {
  createMint,
  getMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
} from "@solana/spl-token";
import {
  findMasterEditionV2Pda,
  findMetadataPda,
} from "@metaplex-foundation/js";
import * as mpl from "@metaplex-foundation/mpl-token-metadata";

describe("Mint the Nft", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Listings as anchor.Program<Listings>;

  let connection;
  let payer;

  it("Setup An account", async () => {
    connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    let accountKey = require(process.env.ANCHOR_WALLET);

    const secretKey = Uint8Array.from(accountKey);
    payer = Keypair.fromSecretKey(secretKey);

    // await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
  });

  it("Mint Nft!", async () => {
    const mint = await createMint(
      connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      0, // We are using 0 to match the CLI decimal default exactly
    );

    console.log("The Mint Account", mint?.toBase58());

    const mintInfo = await getMint(connection, mint);

    console.log("MINT INFO", mintInfo.supply);

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      payer.publicKey,
    );

    console.log(tokenAccount.address.toBase58());

    const tokenAccountInfo = await getAccount(connection, tokenAccount.address);

    console.log(tokenAccountInfo.amount);
    let findNftPda = await anchor.web3.PublicKey.findProgramAddress(
      [mint.toBuffer(), Buffer.from("_nft_listing_data")],
      program.programId,
    );

    const nftPda = findNftPda[0];
    // create the nft listing
    console.log(nftPda.toString());
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
      console.log("Nft Pda Exist", e);
    }

    await mintTo(
      connection,
      payer,
      mint,
      tokenAccount.address,
      nftPda,
      1, // because decimals for the mint are set to 0
    );

    // Set up the metadata using metaplex
    const ON_CHAIN_METADATA = {
      name: "MeMusicNFT",
      symbol: "MNFT",
      uri: "TO_UPDATE_LATER",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    } as mpl.DataV2;

    /// metadata account associated with mint
    // const metadataPDA = await findMetadataPda(mint);

    // console.log("Metadata initialized", metadataPDA.toString());

    // let metadataTransaction = new Transaction().add(
    //   mpl.createCreateMetadataAccountV2Instruction(
    //     {
    //       metadata: metadataPDA,
    //       mint: mint,
    //       mintAuthority: payer.publicKey,
    //       updateAuthority: payer.publicKey,
    //       payer: payer.publicKey,
    //     },
    //     {
    //       createMetadataAccountArgsV2: {
    //         isMutable: false,
    //         data: ON_CHAIN_METADATA,
    //       },
    //     },
    //   ),
    // );

    // await sendAndConfirmTransaction(connection, metadataTransaction, [payer]);

    // const masterEditionPda = await findMasterEditionV2Pda(mint);
    // let masterTransaction = new Transaction().add(
    //   mpl.createCreateMasterEditionV3Instruction(
    //     {
    //       edition: masterEditionPda,
    //       metadata: metadataPDA,
    //       mint: mint,
    //       mintAuthority: payer.publicKey,
    //       updateAuthority: payer.publicKey,
    //       payer: payer.publicKey,
    //     },
    //     {
    //       createMasterEditionArgs: {
    //         maxSupply: 0,
    //       },
    //     },
    //   ),
    // );

    // await sendAndConfirmTransaction(connection, masterTransaction, [payer]);

    // The metaplex metadata make max supply to to limit the supply if that is not set
    // an Alternative would be freezing the mint after the first mint

    // let transaction = new Transaction().add(
    //   createSetAuthorityInstruction(
    //     mint,
    //     payer.publicKey,
    //     AuthorityType.MintTokens,
    //     null,
    //   ),
    // );

    // await sendAndConfirmTransaction(connection, transaction, [payer]);
  });
});
