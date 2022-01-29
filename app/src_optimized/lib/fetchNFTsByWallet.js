// Courtesy of mmdhrumil#9327 on discord

import { programs } from "@metaplex/js/";
import { PublicKey } from "@solana/web3.js";

const {
  metadata: { MetadataData },
  TokenAccount,
} = programs;

export const fetchNFTsOwnedByWallet = async (userWallet, connection) => {
  // console.log(userWallet);
  const accounts = await TokenAccount.getTokenAccountsByOwner(
    connection,
    userWallet
  );
  const accountsWithAmount = accounts
    .map(({ data }) => data)
    .filter(({ amount }) => amount?.toNumber() > 0);

  let nftMintAddresses = accountsWithAmount.map(({ mint }) => mint);

  let nftMetadataAddresses = [];
  let nftAcInfo;

  for (let i = 0; i < nftMintAddresses.length; i++) {
    nftMetadataAddresses[i] = await fetchMetadataAccountForNFT(
      nftMintAddresses[i]
    );

    nftAcInfo = await connection.getMultipleAccountsInfo(
      nftMetadataAddresses,
      "processed"
    );
  }

  let nftAcInfoDeserialized = nftAcInfo
    ?.map((info) =>
      info?.data !== undefined
        ? MetadataData.deserialize(info?.data)
        : undefined
    )
    .filter(function (element) {
      return element !== undefined;
    });

  return nftAcInfoDeserialized;
};

export async function fetchMetadataAccountForNFT(nftMintKey) {
  const metadataBuffer = Buffer.from("metadata");
  const metadataProgramIdPublicKey = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const metadataAccount = (
    await PublicKey.findProgramAddress(
      [
        metadataBuffer,
        metadataProgramIdPublicKey.toBuffer(),
        nftMintKey.toBuffer(),
      ],
      metadataProgramIdPublicKey
    )
  )[0];

  return metadataAccount;
}
