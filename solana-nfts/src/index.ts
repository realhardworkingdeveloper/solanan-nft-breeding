import {
  Connection,
  Keypair,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import secret from "./id.json";

(async () => {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const userKeypair = Keypair.fromSecretKey(new Uint8Array(secret));

  // Airdrop SOL for the user to create the NFT
  const airdropSignature = await connection.requestAirdrop(
    userKeypair.publicKey,
    1 * LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(airdropSignature, "confirmed");

  // Create the Mint Account for the NFT
  const mintAccount = await Token.createMint(
    connection,
    userKeypair,
    userKeypair.publicKey,
    null,
    0,
    TOKEN_PROGRAM_ID
  );

  console.log("-------------mintAccount--------------", mintAccount);

  // Get/Create the Associated Account for the user to hold the NFT
  const userAssosciatedAccount =
    await mintAccount.getOrCreateAssociatedAccountInfo(userKeypair.publicKey);

  // Mint 1 token to the user's associated account
  await mintAccount.mintTo(
    userAssosciatedAccount.address,
    userKeypair.publicKey,
    [],
    1
  );

  // Reset mint_authority to null from the user to prevent further minting
  await mintAccount.setAuthority(
    mintAccount.publicKey,
    null,
    "MintTokens",
    userKeypair.publicKey,
    []
  );

  // Checking balance of the user's associated account
  const accountInfo = await mintAccount.getAccountInfo(
    userAssosciatedAccount.address
  );
  console.log("AccuntInfo", accountInfo)
  console.log("Balance: ", accountInfo.amount.toString());
})();
