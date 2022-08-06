import * as anchor from "@project-serum/anchor";

console.log("mint");

describe("nft", async () => {
  const testNftTitle = "MeMusicNFT";
  const testNftSymbol = "MNFT";
  const testNftUri = "";

  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const idl = require("../target/idl/nft.json");

  const programId = new anchor.web3.PublicKey(
    "DN4P41d49eZosBp6Y4cFsPPmwJFqhm4D759mrSgJGADm",
  );

  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId);

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
  );

  it("Owner Minting to it self!", async () => {
    // Derive the mint address and the associated token account address
    const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

    // get the token address need for the mint
    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey,
    });

    console.log(`New token: ${mintKeypair.publicKey}`);

    // Derive the metadata and master edition addresses

    console.log(`tokenAddress: ${tokenAddress}`);

    // Derive the metadata and master edition addresses

    const metadataAddress = (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mintKeypair.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID,
      )
    )[0];

    console.log("Metadata initialized", metadataAddress.toString());

    const masterEditionAddress = (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mintKeypair.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID,
      )
    )[0];

    console.log(
      "Master edition metadata initialized",
      masterEditionAddress.toString(),
    );

    // Transact with the "mint" function in our on-chain program

    await program.methods
      .mint(testNftTitle, testNftSymbol, testNftUri)
      .accounts({
        masterEdition: masterEditionAddress,
        metadata: metadataAddress,
        mint: mintKeypair.publicKey,
        tokenAccount: tokenAddress,
        mintAuthority: wallet.publicKey,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([mintKeypair])
      .rpc();
  });
});
