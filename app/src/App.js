import "./App.css";
import { useState } from "react";
import { Connection, PublicKey } from "@solana/web3.js";
import { Program, Provider, web3 } from "@project-serum/anchor";
import idl from "./idl.json";
import { Button, Col, Container, Row } from "react-bootstrap";

import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
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
require("@solana/wallet-adapter-react-ui/styles.css");

const wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];

const { SystemProgram, Keypair } = web3;
/* create an account  */
const baseAccount = Keypair.generate();
const opts = {
  preflightCommitment: "processed",
};
const programID = new PublicKey(idl.metadata.address);

const App = () => {
  const [value, setValue] = useState(null);
  const [firstImg, setFirstImg] = useState("");
  const [secImg, setSecImg] = useState("");

  const wallet = useWallet();

  async function getProvider() {
    /* create the provider and return it to the caller */
    /* network set to local network for now */
    const network = "http://127.0.0.1:8899";
    const connection = new Connection(network, opts.preflightCommitment);

    const provider = new Provider(connection, wallet, opts.preflightCommitment);
    return provider;
  }

  const handleImgUpload = async () => {
    if (firstImg && secImg) {
    } else {
      alert("Select two images!");
    }
  };

  async function onImgChange(e, setImg) {
    const file = e.target.files[0];
    let reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => {
      setImg(reader.result);
    };
    reader.onerror = function (error) {
      console.log("Error: ", error);
    };
  }

  if (!wallet.connected) {
    /* If the user's wallet is not connected, display connect wallet button. */
    return (
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          marginTop: "100px",
        }}
      >
        <WalletMultiButton />
      </div>
    );
  } else {
    return (
      <div className="App">
        <Container className="text-center">
          <div className="mt-5 mb-2">
            <h2>Please select images and click submit button.</h2>
          </div>
          <Row className="mt-5">
            <Col md="6">
              <div className="">
                <img src={firstImg} />
                <input type="file" onChange={(e) => onImgChange(e, setFirstImg)} />
              </div>
            </Col>
            <Col md="6">
              <div className="">
                <img src={secImg} />
                <input type="file" onChange={(e) => onImgChange(e, setSecImg)} />
              </div>
            </Col>
          </Row>
          <Row className="mt-5 justify-content-center">
            <Col md="3">
              <Button onClick={handleImgUpload}>Submit</Button>
            </Col>
          </Row>
        </Container>
      </div>
    );
  }
};

/* wallet configuration as specified here: https://github.com/solana-labs/wallet-adapter#setup */
const AppWithProvider = () => (
  <ConnectionProvider endpoint="http://127.0.0.1:8899">
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <App />
      </WalletModalProvider>
    </WalletProvider>
  </ConnectionProvider>
);

export default AppWithProvider;
