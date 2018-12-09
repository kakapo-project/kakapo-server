
import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Step, Transition } from 'semantic-ui-react'

import Dropzone from 'react-dropzone'


const FileUploader = (props) => {

  return (
    <>

      <Dropzone>
        <div style={{ width: '100%', textAlign: 'center' }} >
          <Icon name='cloud upload' size='massive' />
          <Header inverted>Upload File</Header>
        </div>
      </Dropzone>
    </>
  )
}


const UploaderSettings = (props) => {
  return (
    <>
      <Button.Group fluid vertical>
        <Button>Append Data, Ignore on Duplicate</Button>
        <Button>Append Data, Update on Duplicate</Button>
        <Button>Truncate Table and Insert</Button>
      </Button.Group>
    </>
  )
}


class DataImporter extends Component {

  steps = [
    (props) => <FileUploader {...props} />,
    (props) => <UploaderSettings {...props} />,
  ]

  actions = {
  }

  state = {
    step: 0,
  }

  render() {
    return (
      <>
        <Header icon='upload' content='Import Data' />
        <Modal.Content>
          <Segment attached basic style={{ border: '0' }}>
            <Segment attached basic style={{ border: '0' }}>
              { this.steps[this.state.step]({...this.actions, ...this.state}) }
              <Divider hidden />
              { (this.state.step === this.steps.length - 1 ) ?
                <Button
                  positive
                  floated='right'
                  onClick={() => this.props.onComplete(this.state) }
                >
                  Done
                </Button>
                :
                <Button
                  positive
                  floated='right'
                  onClick={() => this.setState({ step: this.state.step + 1 }) }
                >
                  Next
                </Button>
              }
              <Divider hidden clearing />
            </Segment>
          </Segment>
          <Step.Group attached='top'>
            <Step active={this.state.step === 0} >
              <Icon name='upload' />
              <Step.Content>
                <Step.Title>Upload</Step.Title>
                <Step.Description>Upload File</Step.Description>
              </Step.Content>
            </Step>

            <Step active={this.state.step === 1} >
              <Icon name='setting' />
              <Step.Content>
                <Step.Title>Method</Step.Title>
                <Step.Description>How To Import data</Step.Description>
              </Step.Content>
            </Step>
          </Step.Group>
        </Modal.Content>
      </>
    )
  }
}

export default DataImporter