import { Fragment, useState } from 'react';
import { Button, Col, Modal, Row } from 'react-bootstrap';

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
            // fullscreen={true}
            >
                <Modal.Header closeButton>
                    <Modal.Title id="example-custom-modal-styling-title">
                        Your owned NFT list
                    </Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    <Row>
                        {
                            nftLists.map((item, index) => {
                                return (
                                    <Col md="4" key={index}>
                                        <img
                                            src={item.src}
                                            id={item.id}
                                            className='img-fluid img-responsive cusor-pointer'
                                            onClick={() => setParentNft(index, item.id, nftLists)}
                                        />
                                    </Col>
                                )
                            })
                        }
                    </Row>
                </Modal.Body>
            </Modal>
        </Fragment>
    )
}

export default NftListsModal;