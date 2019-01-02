

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import ErrorMsg from '../../ErrorMsg'

import { API_URL } from '../../config'
import { DEFAULT_TYPE, ALL_TYPES } from '../../actions/columns'

import {
  setScriptName,
} from '../../actions/createScript'

import { connect } from 'react-redux'


class CreateScript extends Component {


  render() {

    return (
      <Grid>
        <Grid.Column width={16}>
          <Input
            placeholder='Script Name'
            fluid
            value={this.props.scriptName}
            onChange={(e, data) => this.props.setScriptName(data.value)}
            />
        </Grid.Column>
      </Grid>
    )
  }
}


const mapStateToProps = (state) => ({
  scriptName: state.entityCreator.scriptName,
})

const mapDispatchToProps = (dispatch) => ({
  setScriptName: (name) => dispatch(setScriptName(name)),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateScript)
