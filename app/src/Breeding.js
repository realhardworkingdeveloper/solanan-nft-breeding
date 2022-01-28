import "./App.css";
import { useEffect, useState } from "react";
import { Connection, PublicKey } from "@solana/web3.js";
import { Program, Provider, web3 } from "@project-serum/anchor";
import idl from "./idl.json";
import { Button, Col, Container, Row } from "react-bootstrap";

import nftLists from "./assets/nft-list.json";
import NftListsModal from "./components/nft-lists-modal";

import {
  getPhantomWallet,
  getSlopeWallet,
} from "@solana/wallet-adapter-wallets";
import {
  useWallet,
  WalletProvider,
  ConnectionProvider,
} from "@solana/wallet-adapter-react";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import Timer from "./components/timer";
require("@solana/wallet-adapter-react-ui/styles.css");

const wallets = [
  getPhantomWallet(),
  // getSlopeWallet(),
];

const { SystemProgram, Keypair } = web3;
/* create an account  */
const baseAccount = Keypair.generate();
const opts = {
  preflightCommitment: "processed",
};
const programID = new PublicKey(idl.metadata.address);

const Breeding = () => {
  const [isBreeding, setIsBreeding] = useState(true);
  const [timeRemaining, setTimeRemaining] = useState(300);
  const [balance, setBalance] = useState();

  const [breededAt, setBreededAt] = useState(null);
  const [firstNft, setFirstNft] = useState("");
  const [secNft, setSecNft] = useState("");
  const [showModal, setShowModal] = useState(false);
  const [parent, setParent] = useState("");
  const [filteredNftList, setFilteredNftList] = useState(nftLists);

  useEffect(async () => {
    await initialize();
  }, []);

  const wallet = useWallet();

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const network = web3.clusterApiUrl("devnet");
    const connection = new Connection(network, opts.preflightCommitment);

    const provider = new Provider(connection, wallet, opts.preflightCommitment);
    return provider;
  }

  async function initialize() {
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program(idl, programID, provider);

    try {
      /* interact with the program via rpc */
      const account = await program.account.baseAccount.fetch(
        baseAccount.publicKey
      );
      console.log("account: ", account);
      setTimeRemaining(account.timeRemaining);
      setBreededAt(account.breededAt);
      setFirstNft(account.dataList.firtNft);
      setSecNft(account.dataList.secNft);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  async function breeding() {
    const nftWillBreed = [firstNft, secNft];
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program(idl, programID, provider);
    console.log("================", provider.wallet.publicKey.toString())
    try {
      /* interact with the program via rpc */
      await program.rpc.breeding(nftWillBreed, {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
      });

      const account = await program.account.baseAccount.fetch(
        baseAccount.publicKey
      );
      console.log("account: ", account);
      setBreededAt(account.breededAt);
      setFirstNft(account.dataList.firtNft);
      setSecNft(account.dataList.secNft);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  const handleBreeding = async () => {
    if (firstNft && secNft) {
      await breeding();
    } else {
      alert("Select two NFTs!");
    }
  };

  const selectNft = (parent) => {
    setShowModal(true);
    setParent(parent);
  };

  const setParentNft = (index, id, nftLists) => {
    console.log(index, id, nftLists);
    const selectedNft = filteredNftList[index];
    if (parent == "firstNft") setFirstNft(selectedNft.src);
    else setSecNft(selectedNft.src);
    // setFilteredNftList(nftLists.filter(x => x.id != id))
    setShowModal(false);
  };

  return (
    <div className="App">
      {isBreeding && (<Timer timeRemaining={timeRemaining} />)}
      <div
        style={{
          display: "flex",
          justifyContent: "center",
        }}
      >
        <WalletMultiButton />
      </div>

      <Container className="text-center">
        <div className="mt-3 mb-2">
          <h2>Please select NFTs and click submit button.</h2>
        </div>
        <Row className="mt-5">
          <Col md="6">
            <div className="">
              <img
                src={firstNft}
                className="img-fluid img-thumbnail block-example border border-dark breeded-img"
                onClick={() => selectNft("firstNft")}
              />
              <h3>Parent1</h3>
            </div>
          </Col>
          <Col md="6">
            <div className="">
              <img
                src={secNft}
                className="img-fluid img-thumbnail block-example border border-dark breeded-img"
                onClick={() => selectNft("secNft")}
              />
              <h3>Parent2</h3>
            </div>
          </Col>
        </Row>
        <Row className="mt-2 mb-5 justify-content-center">
          <Col md="3">
            <Button onClick={handleBreeding}>Start Breeding</Button>
          </Col>
        </Row>
      </Container>

      <NftListsModal
        nftLists={filteredNftList}
        showModal={showModal}
        setShowModal={setShowModal}
        setParentNft={setParentNft}
      />
    </div>
  );
};

/* wallet configuration as specified here: https://github.com/solana-labs/wallet-adapter#setup */
const BreedingWithProvider = () => (
  <ConnectionProvider endpoint={web3.clusterApiUrl("devnet")}>
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <Breeding />
      </WalletModalProvider>
    </WalletProvider>
  </ConnectionProvider>
);

export default BreedingWithProvider;
