
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

import { requestingTableData, addRow, deleteRow, modifyValue, copySelection } from '../actions'


import { DataGrid, ContextMenu, NumberFormatter, DefaultFormatter } from '../data-grid/index.js';

class TableData extends Component {

  componentDidMount() {
    this.props.requestingTableData()
  }

  state = {
    topLeft: {
      idx: -1,
      col: -1,
    },
    bottomRight: {
      idx: -1,
      col: -1,
    },
  }

  contexMenuHandler(click, row, col, e) {
    console.log('click: ', click, row, col)

    if (col === null) { // is clicked on row index
      switch (click) {
        case 'delete': return this.props.deleteRow(row)
        case 'add': return this.props.addRow(row)
        case 'cut': return
        case 'copy': return
        case 'paste': return
      }
    } else if (row === null ) { // is clicked on column

    } else {
      switch (click) {
        case 'cut': return
        case 'copy': return this.props.copySelection(this.state, e)
        case 'paste': return
      }
    }
  }

  render() {

    let columnInfo = this.props.columnInfo
    let columns = ['', ...this.props.columns].map((x, idx) => ({
      key: idx,
      name: x,
      editable: x => (x[0] !== '' && idx !== 0),
      frozen: (idx === 0) ? true : false,
      formatter: (columnInfo[x] && columnInfo[x].dataType === 'integer') ? NumberFormatter : DefaultFormatter,
    }))

    let data = [this.props.columns, ...this.props.data].map((x, idx) => [idx || '', ...x])

    return (
      <DataGrid
        columns={columns}
        data={data}
        modifyValue={(rowIdx, colIdx, value) => this.props.modifyValue(rowIdx, colIdx, value)}
        contextMenu={(props) =>
          <ContextMenu
            {...props}
            clickHandler={(click, row, col, e) => this.contexMenuHandler(click, row, col, e)}
          />}
        onSelectionComplete={({topLeft, bottomRight}) => this.setState({topLeft, bottomRight})}
      />
    )
  }
}

const mapStateToProps = (state) => ({
  data: state.table.data,
  columns: state.table.columns,
  columnInfo: state.table.columnInfo,
})

const mapDispatchToProps = (dispatch) => ({
  requestingTableData: () => dispatch(requestingTableData()),
  deleteRow: (idx) => dispatch(deleteRow(idx)),
  addRow: (idx) => dispatch(addRow(idx)),
  modifyValue: (rowIdx, colIdx, value) => dispatch(modifyValue(rowIdx, colIdx, value)),
  copySelection: ({ topLeft, bottomRight }, e) => dispatch(copySelection(topLeft, bottomRight, e)),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TableData)