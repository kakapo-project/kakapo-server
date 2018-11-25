

import React, { Component } from 'react'
import { Button, Modal } from 'semantic-ui-react'

const ErrorMsg = (props) => {
  let types = props.types || ['Continue']
  return (
    <Modal size='tiny' open={props.error !== null} onClose={() => props.onClose()}>
      <Modal.Header>Error Occurred</Modal.Header>
      <Modal.Content>
        <p>{props.error}</p>
      </Modal.Content>
      <Modal.Actions>
        {
          types.map((x, idx) => <Button key={idx} negative onClick={() => props.onClose(x)}>{x}</Button>)
        }
      </Modal.Actions>
    </Modal>
  )
}

export default ErrorMsg