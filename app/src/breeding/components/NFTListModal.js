import { Fragment, useState } from "react";
import { Col, Modal, Row } from "react-bootstrap";
import { NFTItem } from "./NFTItem";

const NftListsModal = (props) => {
  const { nftLists, setParentNft, showModal, setShowModal } = props;

  return (
    <Fragment>
      <Modal
        show={showModal}
        onHide={() => setShowModal(false)}
        dialogClassName="modal-90w"
        aria-labelledby="example-custom-modal-styling-title"
        size="lg"
        scrollable={true}
      >
        <Modal.Header closeButton>
          <Modal.Title id="example-custom-modal-styling-title">
            Select NFT
          </Modal.Title>
        </Modal.Header>
        <Modal.Body>
          {nftLists ? (
            <Row>
              {nftLists.map((item, index) => {
                if (item.data.uri === "") return null;
                return (
                  <Col md="4" key={index}>
                    <NFTItem item={item} setParentNft={setParentNft} />
                  </Col>
                );
              })}
            </Row>
          ) : (
            <p>No NFTs found!</p>
          )}
        </Modal.Body>
      </Modal>
    </Fragment>
  );
};

export default NftListsModal;
