import { useEffect, useState } from "react";
import styled from "styled-components";

import { useWallet } from "@solana/wallet-adapter-react";
import { PublicKey, clusterApiUrl, Connection } from "@solana/web3.js";

import { fetchNFTsOwnedByWallet } from "../lib/fetchNFTsByWallet";

// import { NFTItem } from "./NFTItem";
import BreedingContainer from "./BreedingContainer";

const Container = styled.div`
  display: flex;
  flex-direction: column;
  padding: 1em 0;
`;

const Button = styled.button`
  color: white;
  background-color: teal;
  border: none;
  box-shadow: none;
  padding: 1.5em;
  border-radius: 10px;
  &:hover {
    cursor: pointer;
  }
`;

export function NFTContainer({ network, setIsExpired }) {
  // const { connection } = useConnection();
  const connection = new Connection(clusterApiUrl("devnet"));
  const wallet = useWallet();
  const { publicKey } = wallet;
  const [NFTs, setNFTs] = useState(null);

  async function getNFTList() {
    if (!publicKey) return setNFTs(null);
    let NFTs = await fetchNFTsOwnedByWallet(
      new PublicKey(publicKey),
      connection
    );
    if (typeof NFTs === "undefined") {
      setNFTs(0);
    } else {
      setNFTs(NFTs);
    }
  }

  useEffect(() => {
    getNFTList();
  }, [wallet]);

  if (publicKey) {
    if (NFTs === 0) {
      return (
        <Container>
          <p>
            No NFTs found for <strong>{publicKey.toString()}</strong> on{" "}
            <strong>{network}</strong>!
          </p>
          <Button onClick={getNFTList}>Get NFTs</Button>
        </Container>
      );
    }
    return (
      <Container>
        {/* <Button onClick={getNFTList}>Get NFTs</Button> */}
        <BreedingContainer nftLists={NFTs} setIsExpired={setIsExpired} />
      </Container>
    );
  } else {
    return null;
  }
}
