
import React, { Component } from 'react'
import {
  Button,
  Card,
  Container,
  Divider,
  Dimmer,
  Loader,
  Grid,
  Icon,
  Image,
  Input,
  Label,
  Menu,
  Pagination,
  Segment,
  Sidebar,
  Table
} from 'semantic-ui-react'


import GridLayout from './GridLayout.js'

import Header from '../Header.js'
import ErrorMsg from '../ErrorMsg'


import { WS_URL } from '../config'
import { connect } from 'react-redux'

import { requestingTableData } from '../actions'


class TableData extends Component {

  componentDidMount() {
    this.props.requestingTableData()
  }

  render() {
    return (
      <GridLayout
        data={this.props.data}
        columns={this.props.columns}
        indices={this.props.indices}
        addRow={(afterIdx) => {}}
        updateValue={(input, rowKey, colKey) => {}}
      />
    )
  }
}

const mapStateToProps = (state) => ({
  data: state.table.data,
  columns: state.table.columns,
  indices: state.table.indices,
})

const mapDispatchToProps = (dispatch) => ({
  requestingTableData: () => dispatch(requestingTableData()),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TableData)