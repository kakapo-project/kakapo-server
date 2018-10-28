
import React, { Component } from 'react'
import ReactDOM from 'react-dom'

import { Button, Divider, Header, Icon, Label, Menu, Popup, Portal, Segment, Table } from 'semantic-ui-react'
import ContextMenu from './ContextMenu.js';

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
      <ContextMenu
        key={idx}
        trigger={
          <Table.HeaderCell
              onMouseDown={(e) => this.onMouseDown(e, null, idx)}
              onMouseOver={(e) => this.onMouseOver(e, null, idx)}
              onMouseUp={(e) => this.onMouseUp(e, null, idx)}
          >
            {this.renderColumnIcon(column)}{column.name}
          </Table.HeaderCell>
        }
        position='left bottom'
      >
        <div>
          <Button.Group vertical labeled icon>
            <Button icon='copy' content='Copy' />
            <Button icon='paste' content='Paste' />
            <Button icon='cut' content='Cut' />
            <Divider />
            <Button icon='sort' content='Sort' />
            <Button icon='filter' content='Filter' />
            <Button icon='arrows alternate horizontal' content='Expand' />
            <Button icon='hide' content='Hide' />
          </Button.Group>
        </div>
      </ContextMenu>


    )
  }

  renderRows() {
    let indices = getIndices()
    return indices.map((x, idx) =>
      <ContextMenu
        key={idx}
        trigger={
          <Table.Cell
              key={idx}
              onMouseDown={(e) => this.onMouseDown(e, idx, null)}
              onMouseOver={(e) => this.onMouseOver(e, idx, null)}
              onMouseUp={(e) => this.onMouseUp(e, idx, null)}
          >
            {x}
          </Table.Cell>
        }
        position='right center'
      >
        <div>
          <Button.Group vertical labeled icon>
            <Button icon='copy' content='Copy' />
            <Button icon='paste' content='Paste' />
            <Button icon='cut' content='Cut' />
            <Divider />
            <Button icon='add' content='Add Row' />
            <Button icon='clone' content='Duplicate Row' />
            <Button icon='trash' content='Delete Row' />
          </Button.Group>
        </div>
      </ContextMenu>

    )
  }

  renderData(rowKey, colKey) {
    return (
      <ContextMenu
        key={colKey}
        trigger={
          <Table.Cell
              key={colKey}
              onMouseDown={(e) => this.onMouseDown(e, rowKey, colKey)}
              onMouseOver={(e) => this.onMouseOver(e, rowKey, colKey)}
              onMouseUp={(e) => this.onMouseUp(e, rowKey, colKey)}
              style={{backgroundColor: this.isSelected(rowKey, colKey)? '#EFEFEF': 'white'}}
          >
            {rowKey}|{colKey}
          </Table.Cell>
        }
        position='bottom center'
      >
        <div>
          <Button.Group vertical labeled icon>
            <Button icon='copy' content='Copy' />
            <Button icon='paste' content='Paste' />
            <Button icon='cut' content='Cut' />
          </Button.Group>
        </div>
      </ContextMenu>

    )
  }

  onContextMenu(rowKey, colKey, state = this.state) {
    let newState = {...state, contextMenu: [rowKey, colKey]}
    this.setState(newState)
    return newState
  }

  clearState(state = this.state) {
    let newState = {...state, mouseUp: null, mouseOn: null, mouseDown: null, contextMenu: null}
    this.setState(newState)
    return newState
  }

  onMouseDown(event, rowKey, colKey, state = this.state) {
    if (event.button !== 0) {
      return this.clearState(state)
    }

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

  onMouseOver(event, rowKey, colKey, state = this.state) {
    if (event.button !== 0) {
      return this.clearState(state)
    }

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

  onMouseUp(event, rowKey, colKey, state = this.state) {
    if (event.button !== 0) {
      let newState =  this.clearState(state)
      if (event.button === 2) { //right clicked
        newState = this.onContextMenu(rowKey, colKey, newState)
      }
      return newState
    }

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