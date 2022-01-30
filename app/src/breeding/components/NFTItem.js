import { useState, useEffect } from "react";
import styled from "styled-components";

const ItemContainer = styled.div`
  max-width: 90vw;
`;

const Image = styled.img`
  height: auto;
  max-width: 100%;
`;

export function NFTItem({ item, setParentNft }) {
  const [NFTData, setNFTData] = useState(null);

  useEffect(() => {
    async function getData() {
      let data = await (await fetch(item?.data?.uri)).json();
      // console.log(data);
      setNFTData(data);
    }
    getData();
  }, [item]);
  return (
    <ItemContainer className="text-center">
      <h4>{item.data.name}</h4>
      <Image
        src={NFTData?.image}
        alt="NFT"
        className="img-fluid img-responsive cusor-pointer"
        onClick={() => setParentNft({...item, NFTData})}
      />
    </ItemContainer>
  );
}
