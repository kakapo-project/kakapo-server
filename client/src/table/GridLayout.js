
import React, { Component } from 'react'
import ReactDOM from 'react-dom'

import { Icon, Label, Menu, Table } from 'semantic-ui-react'

import { getColumns, getRows, getIndices } from './actions.js'

import DataGrid from '../data-grid'

class GridLayout extends Component {

  state = {
    columnDefs: [
        {headerName: "Make", field: "make"},
        {headerName: "Model", field: "model"},
        {headerName: "Price", field: "price"}

    ],
    rowData: [
        {make: "Toyota", model: "Celica", price: 35000},
        {make: "Ford", model: "Mondeo", price: 32000},
        {make: "Porsche", model: "Boxter", price: 72000}
    ]
  }


  renderTypeSymbol(type) {
    switch (type) {
      case 'Boolean':
        return '<i aria-hidden="true" class="check icon">'
      case 'String':
        return '<i aria-hidden="true" class="font icon">'
      case 'Integer':
        return '<i aria-hidden="true" class="hashtag icon">'
      case 'Number':
        return '<i aria-hidden="true" class="times icon">'
      case 'Percentage':
        return '<i aria-hidden="true" class="percent icon">'
      case 'Money':
        return '<i aria-hidden="true" class="dollar icon">'
      case 'Date':
        return '<i aria-hidden="true" class="calendar icon">'
      case 'DateTime':
        return '<i aria-hidden="true" class="clock icon">'
      case 'Json':
        return '{}'
      default:
        return ''
    }
  }

  renderColumnIcon(column) {
    if (column.isPrimaryKey) {
      return <Icon name='key' />
    } else if (column.isForeignKey) {
      return <Icon name='linkify' />
    } else {
      return <></>
    }
  }

  renderIndexForRowWithNoKey() {
    return '<i aria-hidden="true" class="question icon">'
  }

  renderColumns() {

    let columns = getColumns()

    return columns.map((column, idx) =>
      <Table.HeaderCell
          key={idx}
          onMouseDown={(e) => this.onMouseDown(null, idx)}
          onMouseOver={(e) => this.onMouseOver(null, idx)}
          onMouseUp={(e) => this.onMouseUp(null, idx)}
      >
        {this.renderColumnIcon(column)}{column.name}
      </Table.HeaderCell>,
    )
  }

  renderRows() {
    let indices = getIndices()
    return indices.map((x, idx) =>
      <Table.Cell
          key={idx}
          onMouseDown={(e) => this.onMouseDown(idx, null)}
          onMouseOver={(e) => this.onMouseOver(idx, null)}
          onMouseUp={(e) => this.onMouseUp(idx, null)}
      >
        {x}
      </Table.Cell>
    )
  }

  renderData(rowKey, colKey) {
    return (
      <Table.Cell
          key={colKey}
          onMouseDown={(e) => this.onMouseDown(rowKey, colKey)}
          onMouseOver={(e) => this.onMouseOver(rowKey, colKey)}
          onMouseUp={(e) => this.onMouseUp(rowKey, colKey)}
          style={{backgroundColor: this.isSelected(rowKey, colKey)? '#A0A0A0': '#EFEFEF'}}
      >
        {rowKey}|{colKey}
      </Table.Cell>
    )
  }

  onMouseDown(rowKey, colKey, state = this.state) {
    let newState = {
      ...state,
      mouseDown: [
        (rowKey === null) ? 0 : rowKey,
        (colKey === null) ? 0 : colKey,
      ],
      mouseOn: [
        (rowKey === null) ? Number.MAX_SAFE_INTEGER  : rowKey,
        (colKey === null) ? Number.MAX_SAFE_INTEGER : colKey,
      ],
      mouseUp: null
    }
    this.setState(newState)
    return newState
  }

  onMouseOver(rowKey, colKey, state = this.state) {
    if (!this.state.mouseUp) {
      let newState = {
        ...state,
        mouseOn: [
          (rowKey === null) ? Number.MAX_SAFE_INTEGER : rowKey,
          (colKey === null) ? Number.MAX_SAFE_INTEGER : colKey,
        ]
      }
      this.setState(newState)
      return newState
    } else {
      return state
    }
  }

  onMouseUp(rowKey, colKey, state = this.state) {
    let newState = {
      ...state,
      mouseUp: [
        (rowKey === null) ? Number.MAX_SAFE_INTEGER : rowKey,
        (colKey === null) ? Number.MAX_SAFE_INTEGER : colKey,
      ],
      mouseOn: null
    }
    this.setState(newState)
    return newState
  }

  isSelected(rowKey, colKey) {
    let initial = this.state.mouseDown
    if (!initial) {
      return false
    }

    let final = this.state.mouseOn || this.state.mouseUp

    if (
      initial[0] >= rowKey && final[0] <= rowKey &&
      initial[1] >= colKey && final[1] <= colKey
    ) {
      return true
    }

    if (
      initial[0] >= rowKey && final[0] <= rowKey &&
      initial[1] <= colKey && final[1] >= colKey
    ) {
      return true
    }

    if (
      initial[0] <= rowKey && final[0] >= rowKey &&
      initial[1] >= colKey && final[1] <= colKey
    ) {
      return true
    }

    if (
      initial[0] <= rowKey && final[0] >= rowKey &&
      initial[1] <= colKey && final[1] >= colKey
    ) {
      return true
    }

    return false
  }

  render() {

    return (
      <div
        className="ag-theme-balham"
        style={{
          height: '100%',
          width: '100%',
        }}
      >
          <DataGrid
            columns={this.renderColumns()}
            rows={this.renderRows()}
            getData={(rowKey, colKey) => this.renderData(rowKey, colKey)}
          />
      </div>
    )
  }
}

export default GridLayout;