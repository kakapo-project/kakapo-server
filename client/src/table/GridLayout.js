
import React, { Component } from 'react'
import ReactDOM from 'react-dom'

import { Button, Divider, Header, Icon, Input, Label, Menu, Popup, Portal, Segment, Table } from 'semantic-ui-react'
import ContextMenu from 'semantic-ui-react-context-menu'

import DataGrid from '../data-grid'

import _ from 'lodash'


class GridLayout extends Component {

  state = {
  }

  getColumns() {
    let columns = this.props.columns
    return columns.map(x => ({
      name: x,
      dataType: 'String', //TODO
      isForeignKey: false,
      isPrimaryKey: false,
    }))
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

  isColumnContextMenu(idx) {
    return (
      this.state.contextMenu &&
      this.state.contextMenu[0] === null &&
      this.state.contextMenu[1] === idx
    )
  }

  isIndexContextMenu(idx) {
    return (
      this.state.contextMenu &&
      this.state.contextMenu[0] === idx &&
      this.state.contextMenu[1] === null
    )
  }

  isDataContextMenu(rowIdx, colIdx) {
    return (
      this.state.contextMenu &&
      this.state.contextMenu[0] === rowIdx &&
      this.state.contextMenu[1] === colIdx
    )
  }

  clearContextMenu(e) {
    if (e.button === 0) {
      this.setState({ contextMenu: null })
    }
  }

  addRow(afterIdx) {
    this.props.addRow(afterIdx)
    this.setState({ contextMenu: null })
  }

  isBoxSelected(rowKey, colKey) {
    return (
      this.state.boxSelected &&
      (this.state.boxSelected[0] == rowKey) &&
      (this.state.boxSelected[1] == colKey)
    )
  }


  doubleClickHandler(cb, ...args) {
    if (!this._delayedClick) {
      this._delayedClick = _.debounce(() => {
        this._clickedOnce = false
      }, 300)
    }
    if (this._clickedOnce) {
      this._delayedClick.cancel()
      this._clickedOnce = false
      cb(...args)
    } else {
      this._delayedClick()
      this._clickedOnce = true
    }
  }

  onMouseDown(event, rowKey, colKey) {
    if (event.button !== 0) {
      this.setState({
        mouseUp: null,
        mouseOn: null,
        mouseDown: null,
        contextMenu: [ rowKey, colKey ],
      })
    } else {

      let newState = {}

      this.doubleClickHandler((rowKey, colKey) => {
        newState = { ...newState, boxSelected: [rowKey, colKey] }
      }, rowKey, colKey)

      let isBoxSelected = this.isBoxSelected(rowKey, colKey)
      let inputValueStateChange = {}
      if (!isBoxSelected) {
        inputValueStateChange = this.getInputValueStateChange()
      }

      newState = {
        mouseDown: [
          (rowKey === null) ? 0 : rowKey,
          (colKey === null) ? 0 : colKey,
        ],
        mouseOn: [
          (rowKey === null) ? Number.MAX_SAFE_INTEGER  : rowKey,
          (colKey === null) ? Number.MAX_SAFE_INTEGER : colKey,
        ],
        mouseUp: null,
        contextMenu: null,
        ...inputValueStateChange,
        ...newState,
      }

      this.setState(newState)
    }
  }

  onMouseOver(event, rowKey, colKey) {
    if (event.button !== 0) {
      this.setState({
        mouseUp: null,
        mouseOn: null,
        mouseDown: null,
      })
    } else {
      if (!this.state.mouseUp) {
        this.setState({
          mouseOn: [
            (rowKey === null) ? Number.MAX_SAFE_INTEGER : rowKey,
            (colKey === null) ? Number.MAX_SAFE_INTEGER : colKey,
          ],
        })
      }
    }
  }

  onMouseUp(event, rowKey, colKey) {
    if (event.button !== 0) {
      this.setState({
        mouseUp: null,
        mouseOn: null,
        mouseDown: null,
      })
    } else {
      this.setState({
        mouseUp: [
          (rowKey === null) ? Number.MAX_SAFE_INTEGER : rowKey,
          (colKey === null) ? Number.MAX_SAFE_INTEGER : colKey,
        ],
        mouseOn: null,
      })
    }
  }

  changeInputValue(data, rowKey, colKey) {
    this.setState({ inputValue: data })
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

  getInputValueStateChange() {

    if (this.isInputValueSet()) {
      this.props.updateValue(
        this.state.inputValue,
        this.state.boxSelected[0],
        this.state.boxSelected[1],
      )
    }

    return {
      boxSelected: null,
      inputValue: null,
    }
  }

  isInputValueSet() {
    return (this.state.inputValue === '') || this.state.inputValue
  }

  onBlurTable() {
    if (!this.state.contextMenu) { //only blur if the context menu is not open

      let inputValueStateChange = this.getInputValueStateChange()

      this.setState({
        mouseUp: null,
        mouseOn: null,
        mouseDown: null,
        contextMenu: null,
        ...inputValueStateChange,
      })
    }
  }

  renderColumns() {
    let columns = this.getColumns()
    return columns.map((column, idx) =>
      <ContextMenu
        key={idx}
        trigger={
          <Table.HeaderCell
              onMouseDown={(e) => this.onMouseDown(e, null, idx)}
              onMouseOver={(e) => this.onMouseOver(e, null, idx)}
              onMouseUp={(e) => this.onMouseUp(e, null, idx)}
              style={{
                textAlign: (column.dataType == 'Integer') ? 'right' : 'left'
              }}
          >
            {this.renderColumnIcon(column)}{column.name}
          </Table.HeaderCell>
        }
        items={[
          { key: 0, icon: <Icon name='sort'/>, content: 'Sort' },
          { key: 1, icon: <Icon name='filter'/>, content: 'Filter' },
          { key: 2, icon: <Icon name='arrows alternate horizontal'/>, content: 'Expand' },
          { key: 3, icon: <Icon name='hide' />, content: 'Hide' },
          { key: 4, icon: <Icon name='cancel' /> },
        ]}
        onClick={(e, item) => {
          console.log('context menu click item', item)
        }}
      />
    )
  }

  renderRows() {
    let indices = this.props.indices
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
        items={[
          { key: 0, icon: <Icon name='copy'/>, content: 'Copy' },
          { key: 1, icon: <Icon name='paste'/>, content: 'Paste' },
          { key: 2, icon: <Icon name='cut'/>, content: 'Cut' },
          //TODO: maybe put a divider here
          { key: 3, icon: <Icon name='add' />, content: 'Add Row' },
          { key: 4, icon: <Icon name='clone'/>, content: 'Duplicate Row' },
          { key: 5, icon: <Icon name='trash'/>, content: 'Delete Row' },
          { key: 6, icon: <Icon name='cancel'/> },
        ]}
        onClick={(e, item) => {
          console.log('context menu click item', item)
          switch (item.key) {
            case 3:
              this.addRow(idx)
              return
          }
        }}
      />

    )
  }

  renderData(rowKey, colKey) {
    let data = this.props.data[rowKey][colKey]
    let isDataSelected = this.isBoxSelected(rowKey, colKey)
    let style = {
      backgroundColor: this.isSelected(rowKey, colKey)? '#EFEFEF': 'white',
      textAlign: 'left', //(dataType === 'Integer') ? 'right' : 'left'
    }
    if (isDataSelected) {
      style.padding = 0
    }

    return (
      <ContextMenu
        key={colKey}
        trigger={
          <Table.Cell
              key={colKey}
              onMouseDown={(e) => this.onMouseDown(e, rowKey, colKey)}
              onMouseOver={(e) => this.onMouseOver(e, rowKey, colKey)}
              onMouseUp={(e) => this.onMouseUp(e, rowKey, colKey)}
              style={style}
          >
            { isDataSelected ?
              <Input
                ref={(c) => c && c.focus()}
                value={this.isInputValueSet() ? this.state.inputValue : data}
                style={{width: '100%'}}
                onChange={(e, data) => this.changeInputValue(data.value, rowKey, colKey)}
              />
              :
              `${data}`
            }
          </Table.Cell>
        }
        items={[
          { key: 0, icon: <Icon name='copy'/>, content: 'Copy' },
          { key: 1, icon: <Icon name='paste'/>, content: 'Paste' },
          { key: 2, icon: <Icon name='cut'/>, content: 'Cut' },
          { key: 6, icon: <Icon name='cancel'/> },
        ]}
        onClick={(e, item) => {
          console.log('context menu click item', item)
        }}
      />

    )
  }

  render() {

    if (this.props.data === null || this.props.columns === null) {
      return <div />
    }

    //This is a hack for getting onBlur to work for divs
    const onBlur = (e) => {
      var currentTarget = e.currentTarget;

      setTimeout(() => {
        if (!currentTarget.contains(document.activeElement)) {
          this.onBlurTable()
        }
      }, 0);
    }

    return (
      <div
        tabIndex='1'
        onBlur={(e) => onBlur(e)}
        style={{ outline: 'none' }}
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