import React from "react";
import ReactDOM from "react-dom";
import Main from "./Main";

import 'bootstrap/dist/css/bootstrap.min.css';
require("./styles/index.css");
require("@solana/wallet-adapter-react-ui/styles.css");

const BreedingGroup = ({setIsExpired}) => {
  return (<Main setIsExpired={setIsExpired} />)
}

export default BreedingGroup;
