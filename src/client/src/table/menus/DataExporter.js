
import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Step, Transition } from 'semantic-ui-react'


const FileExporter = (props) => {

  return (
    <>
      <Input
        placeholder='File Name'
        fluid
        value={props.fileName}
        onChange={(e, {value}) => props.setFilename(value)}
      />
      <Divider hidden />
      <Button.Group fluid>
        <Button
          active={props.fileType === '.xlsx'}
          onClick={() => this.props.setFiletype('.xlsx')}
        >.xlsx</Button>
        <Button
          active={props.fileType === '.csv'}
          onClick={() => props.setFiletype('.csv')}
        >.csv</Button>
        <Button
          active={props.fileType === '.tsv'}
          onClick={() => props.setFiletype('.tsv')}
        >.tsv</Button>
      </Button.Group>
    </>
  )
}


const RowsExporter = (props) => {
  return (
    <>
      <Button.Group fluid>
        <Button>Export All</Button>
        <Button>Export Selected</Button>
      </Button.Group>

      {
        (props.fileType === '.csv' || props.fileType === '.tsv') ?
          <>
            <Header inverted>Date Format</Header>
            <Button.Group vertical>
              <Button>YYYY-MM-DD</Button>
              <Button>YYYY/MM/DD</Button>
              <Button>Month DD,YYYY</Button>
            </Button.Group>
          </>
          :
          <>
            <Header inverted>Sheet Name</Header>
            <Input placeholder='Sheet Name' />
          </>
      }
    </>
  )
}


const DownloadExporter = (props) => {
  return (
    <>
      <Button fluid icon labelPosition='left'>
        <Icon name='arrow alternate circle down' />
        Download
      </Button>
      <Divider hidden />
      <Input fluid icon='at' iconPosition='left' placeholder='Email' action='Send' />
    </>
  )
}


class DataExporter extends Component {

  steps = [
    (props) => <FileExporter {...props} />,
    (props) => <RowsExporter {...props} />,
    (props) => <DownloadExporter {...props} />,
  ]

  actions = {
    setFilename: (value) => this.setState({ fileName: value }),
    setFiletype: (value) => this.setState({ fileType: value }),
  }

  state = {
    step: 0,
    fileName: '',
    fileType: '.xlsx',
  }

  render() {
    return (
      <>
        <Header icon='download' content='Export Data' />
        <Modal.Content>
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
          <Step.Group attached='top'>
            <Step active={this.state.step === 0} >
              <Icon name='file excel' />
              <Step.Content>
                <Step.Title>File</Step.Title>
                <Step.Description>Filename and File type</Step.Description>
              </Step.Content>
            </Step>

            <Step active={this.state.step === 1} >
              <Icon name='table' />
              <Step.Content>
                <Step.Title>Rows</Step.Title>
                <Step.Description>What to export</Step.Description>
              </Step.Content>
            </Step>

            <Step active={this.state.step === 2} >
              <Icon name='arrow alternate circle down' />
              <Step.Content>
                <Step.Title>Download</Step.Title>
              </Step.Content>
            </Step>
          </Step.Group>
        </Modal.Content>
      </>
    )
  }
}

export default DataExporter