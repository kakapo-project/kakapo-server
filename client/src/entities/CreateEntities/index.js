

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import ErrorMsg from '../../ErrorMsg'

import { API_URL } from '../../config'
import { DEFAULT_TYPE, ALL_TYPES } from '../../actions/columns'

import CreateTable from './CreateTable'

import { setTableName, commitTableChanges, exitCreatingEntities, closeEntityCreatorErrorMessage, startCreatingEntities, pullData } from '../../actions'
import { connect } from 'react-redux'


class CreateEntities extends Component {


  handleCreatedEntities(commitChanges = false) {
    if (commitChanges) {
      this.props.commitChanges()
    } else {
      this.props.exitCreatingEntities()
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
            <Button icon='database' content='Table' />
            <Button icon='search' content='Query' />
            <Button icon='code' content='Script' />
          </Button.Group>
        </Segment>
        <Header icon='database' content='Create New Table' />
        <Modal.Content>
          <ErrorMsg error={this.props.error} onClose={(type) => this.props.closeErrorMessage()}/>

          <CreateTable />

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
})

const mapDispatchToProps = (dispatch) => ({
  commitChanges: () => dispatch(commitTableChanges()),
  exitCreatingEntities: () => dispatch(exitCreatingEntities()),
  closeErrorMessage: () => dispatch(closeEntityCreatorErrorMessage()),
  startCreatingEntities: () => dispatch(startCreatingEntities()),
  pullData: () => dispatch(pullData()),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateEntities)
