

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import ErrorMsg from '../../ErrorMsg'

import { API_URL } from '../../config'
import { DEFAULT_TYPE, ALL_TYPES } from '../../actions/columns'

import CreateTable from './CreateTable'
import CreateScript from './CreateScript'
import CreateQuery from './CreateQuery'


import { setTableName, commitChanges, exitCreatingEntities, closeEntityCreatorErrorMessage, startCreatingEntities, pullData, setMode } from '../../actions'
import { connect } from 'react-redux'


class CreateEntities extends Component {

  handleCreatedEntities(commitChanges = false) {
    if (commitChanges) {
      this.props.commitChanges()
    } else {
      this.props.exitCreatingEntities()
    }
  }

  getEntityCreatorComponent() {
    switch (this.props.mode) {
      case 'Table':
        return <CreateTable />
      case 'Query':
        return <CreateQuery />
      case 'Script':
        return <CreateScript />
    }
  }

  render() {

    return (
      <Modal
        trigger={<Button
          circular
          positive
          icon='plus'
          floated='right'
          size='massive'
          onClick={e =>this.props.startCreatingEntities()}
        />}
        open={this.props.creatingEntities}
        onClose={e => this.handleCreatedEntities()}
        basic
      >
        <Segment basic padded>
          <Button.Group fluid>
            <Button icon='database' content='Table'
              active={this.props.mode === 'Table'}
              onClick={() => this.props.setMode('Table')} />
            <Button icon='search' content='Query'
              active={this.props.mode === 'Query'}
              onClick={() => this.props.setMode('Query')} />
            <Button icon='code' content='Script'
              active={this.props.mode === 'Script'}
              onClick={() => this.props.setMode('Script')} />
          </Button.Group>
        </Segment>
        <Header
          icon={(this.props.mode === 'Table') ? `database` : (this.props.mode === 'Query') ? 'search' : 'code'}
          content={`Create New ${this.props.mode}`}
        />
        <Modal.Content>
          <ErrorMsg error={this.props.error} onClose={(type) => this.props.closeErrorMessage()}/>

          { this.getEntityCreatorComponent() }

          <Divider hidden/>
          <Divider />
          <Divider hidden/>

          <Button color='green' onClick={e => this.handleCreatedEntities(true)} inverted floated='right'>
            <Icon name='checkmark' />Create
          </Button>
        </Modal.Content>
      </Modal>
    )
  }
}


const mapStateToProps = (state) => ({
  creatingEntities: state.entityCreator.creatingEntities,
  error: state.entityCreator.error,
  mode: state.entityCreator.mode,
})

const mapDispatchToProps = (dispatch) => ({
  commitChanges: () => dispatch(commitChanges()),
  exitCreatingEntities: () => dispatch(exitCreatingEntities()),
  closeErrorMessage: () => dispatch(closeEntityCreatorErrorMessage()),
  startCreatingEntities: () => dispatch(startCreatingEntities()),
  pullData: () => dispatch(pullData()),

  setMode: (mode) => dispatch(setMode(mode)),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateEntities)
