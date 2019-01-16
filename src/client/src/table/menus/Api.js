

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Step, Transition } from 'semantic-ui-react'



class Api extends Component {

  render() {
    return (
      <>
        <Header icon='anchor' content='API Documentation' />
        <Modal.Content>
          <Segment attached basic style={{ border: '0' }}>
            GET
          </Segment>
        </Modal.Content>
      </>
    )
  }
}

export default Api