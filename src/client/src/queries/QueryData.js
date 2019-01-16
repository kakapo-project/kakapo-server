


import React, { Component } from 'react'
import {
  Button,
  Card,
  Container,
  Divider,
  Dimmer,
  Dropdown,
  Loader,
  Grid,
  Icon,
  Image,
  Input,
  Label,
  Pagination,
  Segment,
  Sidebar,
  Table
} from 'semantic-ui-react'



import Header from '../Header.js'
import ErrorMsg from '../ErrorMsg'


import { WS_URL } from '../config'
import { connect } from 'react-redux'

import { requestingTableData, addRow, deleteRow, modifyValue } from '../actions'


import { DataGrid, ContextMenu, NumberFormatter, DefaultFormatter } from '../data-grid/index.js';

class QueryData extends Component {

  render() {

    let columns = ['', ...this.props.columns].map((x, idx) => ({
      key: idx,
      name: x,
      editable: false,
      frozen: (idx === 0) ? true : false,
      formatter: NumberFormatter,
    }))

    let data = [this.props.columns, ...this.props.data].map((x, idx) => [idx || '', ...x])

    return (
      <DataGrid
        columns={columns}
        data={data}
        modifyValue={null}
        contextMenu={null}
        contextMenuProps={null}
      />
    )
  }
}

export default QueryData