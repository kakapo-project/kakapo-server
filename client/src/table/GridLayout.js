
import React, { Component } from 'react'
import ReactDOM from 'react-dom'

import { Button, Divider, Header, Icon, Label, Menu, Popup, Portal, Segment, Table } from 'semantic-ui-react'
import ContextMenu from './ContextMenu.js';

import DataGrid from '../data-grid'

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

  renderIndexForRowWithNoKey() {
    return '<i aria-hidden="true" class="question icon">'
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
        position='bottom left'
        open={this.isColumnContextMenu(idx)}
        onClose={(e) => this.clearContextMenu(e)}
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
        position='right center'
        open={this.isIndexContextMenu(idx)}
        onClose={(e) => this.clearContextMenu(e)}
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
    let data = this.props.data[rowKey][colKey]
    return (
      <ContextMenu
        key={colKey}
        trigger={
          <Table.Cell
              key={colKey}
              onMouseDown={(e) => this.onMouseDown(e, rowKey, colKey)}
              onMouseOver={(e) => this.onMouseOver(e, rowKey, colKey)}
              onMouseUp={(e) => this.onMouseUp(e, rowKey, colKey)}
              style={{
                backgroundColor: this.isSelected(rowKey, colKey)? '#EFEFEF': 'white',
                textAlign: 'left', //(dataType === 'Integer') ? 'right' : 'left'
              }}
          >
            {`${data}`}
          </Table.Cell>
        }
        position='bottom center'
        open={this.isDataContextMenu(rowKey, colKey)}
        onClose={(e) => this.clearContextMenu(e)}
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

  onMouseDown(event, rowKey, colKey) {
    if (event.button !== 0) {
      this.setState({
        mouseUp: null,
        mouseOn: null,
        mouseDown: null,
        contextMenu: [ rowKey, colKey ],
      })
    } else {
      this.setState({
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
      })
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

  onBlurTable() {
    console.log('blurred')
    if (!this.state.contextMenu) { //only blur if the context menu is not open
      this.setState({
        mouseUp: null,
        mouseOn: null,
        mouseDown: null,
        contextMenu: null
      })
    }

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