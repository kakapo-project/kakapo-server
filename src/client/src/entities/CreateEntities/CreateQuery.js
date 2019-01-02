

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import ErrorMsg from '../../ErrorMsg'

import { API_URL } from '../../config'
import { DEFAULT_TYPE, ALL_TYPES } from '../../actions/columns'

import {
  setQueryName,
} from '../../actions/createQuery'

import { connect } from 'react-redux'


class CreateQuery extends Component {


  render() {

    return (
      <Grid>
        <Grid.Column width={16}>
          <Input
            placeholder='Query Name'
            fluid
            value={this.props.queryName}
            onChange={(e, data) => this.props.setQueryName(data.value)}
            />
        </Grid.Column>
      </Grid>
    )
  }
}


const mapStateToProps = (state) => ({
  queryName: state.entityCreator.queryName,
})

const mapDispatchToProps = (dispatch) => ({
  setQueryName: (name) => dispatch(setQueryName(name)),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateQuery)
