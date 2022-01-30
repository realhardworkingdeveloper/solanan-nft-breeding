import styled from "styled-components";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

const StyledHeader = styled.div`
  display: flex;
  justify-content: center;
  width: 100%;
  min-height: 50px;
  background-color: var(--black);
`;

const InnerContainer = styled.nav`
  display: flex;
  justify-content: end;
  align-items: stretch;
  /* max-width: 1000px; */
  width: 100%;
  padding: 0 2em;
`;

const LeftNav = styled.ul`
  list-style: none;
  display: flex;
  flex-direction: row;
  padding: 0;
`;

const RightNav = styled.ul`
  list-style: none;
  display: flex;
  flex-direction: row;
`;

const NavItem = styled.button`
  border: none;
  font-size: 1em;
  color: white;
  background-color: var(--purple);
  padding: 1.3em;
  font-weight: 800;
  border-radius: 5px;
  :hover {
    cursor: pointer;
  }
`;

export function Header() {
  return (
    <StyledHeader>
      <InnerContainer>
        <RightNav>
          <li>
            <WalletMultiButton />
          </li>
        </RightNav>
      </InnerContainer>
    </StyledHeader>
  );
}
