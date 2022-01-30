import { useState } from "react";

import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { WalletConnectionProvider } from "./components/WalletConnectionProvider";

import { NFTContainer } from "./components/NFTContainer";

import { OuterContainer, Container } from "./styles/common";
import { Header } from "./components/Header";

function Main({setIsExpired}) {
  // eslint-disable-next-line no-unused-vars
  const [network, setNetwork] = useState("devnet");
  return (
    <WalletConnectionProvider network={network}>
      <WalletModalProvider>
        <OuterContainer>
          <Header />
          <Container className="App">
            <NFTContainer network={network} setIsExpired={setIsExpired} />
          </Container>
        </OuterContainer>
      </WalletModalProvider>
    </WalletConnectionProvider>
  );
}

export default Main;
