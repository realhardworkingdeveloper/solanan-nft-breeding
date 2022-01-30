import "../styles/BreedingContainer.css";
import { useEffect, useState } from "react";

import { Button, Col, Container, Row } from "react-bootstrap";

import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import { Program, Provider, web3 } from "@project-serum/anchor";
import { useWallet } from "@solana/wallet-adapter-react";

import NftListsModal from "./NFTListModal";

import Timer from "./Timer";
import idl from "../idl.json";
import key from "../key.json";

const { SystemProgram, Keypair } = web3;
/* create an account  */
const baseAccount = Keypair.fromSecretKey(new Uint8Array(key))
const opts = {
  preflightCommitment: "processed",
};
const programID = new PublicKey(idl.metadata.address);

const BreedingContainer = ({ nftLists, setIsExpired }) => {
  const [isBreeding, setIsBreeding] = useState(false);
  const [timeRemaining, setTimeRemaining] = useState(5);
  const [isRequested, setIsRequested] = useState(false);
  const [isCreated, setIsCreated] = useState(false);

  const [firstNft, setFirstNft] = useState(null);
  const [secNft, setSecNft] = useState(null);
  const [showModal, setShowModal] = useState(false);
  const [parent, setParent] = useState("");

  const wallet = useWallet();

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */

    // const network = clusterApiUrl("devnet");
    const network = "http://127.0.0.1:8899";
    const connection = new Connection(network, opts.preflightCommitment);

    const provider = new Provider(connection, wallet, opts.preflightCommitment);
    return provider;
  }

  async function breedingStart() {
    setIsBreeding(true);
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program(idl, programID, provider);
    try {
      /* interact with the program via rpc */
      await program.rpc.create({
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
      setIsRequested(account.is_requested);
      setIsCreated(account.is_created);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  async function fetchTimeRemaing() {
    const provider = await getProvider();
    /* create the program interface combining the idl, program ID, and provider */
    const program = new Program(idl, programID, provider);
    try {
      /* interact with the program via rpc */
      await program.rpc.fetchTimeRamaing();

      const account = await program.account.baseAccount.fetch(
        baseAccount.publicKey
      );
      setIsRequested(account.is_requested);
      setIsCreated(account.is_created);
      setTimeRemaining(account.time_remaining);
    } catch (err) {
      console.log("Transaction error: ", err);
    }
  }

  const handleBreedingStart = async () => {
    if (firstNft && secNft) {
      await breedingStart();
    } else {
      alert("Select two NFTs!");
    }
  };

  const selectNft = (parent) => {
    setShowModal(true);
    setParent(parent);
  };

  const setParentNft = (selectedItem) => {
    if (parent == "firstNft") setFirstNft(selectedItem);
    else setSecNft(selectedItem);
    setShowModal(false);
  };

  const onCompleteBrReq = () => {
    setIsBreeding(false);
    setIsExpired(true)
  }

  return (
    <div className="App">
      {isBreeding && <Timer timeRemaining={timeRemaining} onComplete={() => onCompleteBrReq()} />}

      <Container className="text-center">
        <div>
          <h2>Please select NFTs and click submit button.</h2>
        </div>
        <Row className="mt-3">
          <Col md="6">
            <div className="">
              <img
                src={firstNft?.NFTData?.image}
                className="img-fluid img-thumbnail block-example border border-dark breeded-img"
                onClick={() => selectNft("firstNft")}
              />
              <h3>Parent1</h3>
            </div>
          </Col>
          <Col md="6">
            <div className="">
              <img
                src={secNft?.NFTData?.image}
                className="img-fluid img-thumbnail block-example border border-dark breeded-img"
                onClick={() => selectNft("secNft")}
              />
              <h3>Parent2</h3>
            </div>
          </Col>
        </Row>
        <Row className="mt-2 justify-content-center">
          <Col md="3">
            <Button onClick={handleBreedingStart}>Start Breeding</Button>
          </Col>
        </Row>
      </Container>

      <NftListsModal
        nftLists={nftLists}
        showModal={showModal}
        setShowModal={setShowModal}
        setParentNft={setParentNft}
      />
    </div>
  );
};

export default BreedingContainer;
