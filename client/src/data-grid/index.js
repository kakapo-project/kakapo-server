

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


import { Menu } from 'react-data-grid-addons'

import { hide } from './contextMenuHelper'
import InnerDataGrid from './InnerDatagrid';

const ColumnContextMenu = ({id, col, ...props}) => (
  <Menu.ContextMenu id={id} className={'ui active visible dropdown'}>
    <Dropdown.Menu className={'visible'} style={{top: -70 /*needed because the context menu doesn't work properly*/}}>
      <Dropdown.Item icon='sort' text='Sort' />
      <Dropdown.Item icon='filter' text='Filter' />
      <Dropdown.Item icon='arrows alternate horizontal' text='Expand' />
      <Dropdown.Item icon='hide' text='Hide' />
      <Dropdown.Divider />
      <Dropdown.Item icon='cut' text='Cut' />
      <Dropdown.Item icon='copy' text='Copy' />
      <Dropdown.Item icon='paste' text='Paste' onClick={(e) => hide()} />
    </Dropdown.Menu>
  </Menu.ContextMenu>
)

const IndexContextMenu = ({id, row, ...props}) => (
  <Menu.ContextMenu id={id} className={'ui active visible dropdown'}>
    <Dropdown.Menu className={'visible'} style={{top: -70 /*needed because the context menu doesn't work properly*/}}>
      <Dropdown.Item icon='add' text='Add Row' onClick={(e) => { props.onRowAdded(row); hide() }} />
      <Dropdown.Item icon='clone' text='Duplicate Row' />
      <Dropdown.Item icon='trash' text='Delete Row' onClick={(e) => { props.onRowDelete(row); hide() }} />
      <Dropdown.Divider />
      <Dropdown.Item icon='cut' text='Cut' />
      <Dropdown.Item icon='copy' text='Copy' />
      <Dropdown.Item icon='paste' text='Paste' onClick={(e) => hide()} />
    </Dropdown.Menu>
  </Menu.ContextMenu>
)

const CellContextMenu = ({id, col, row, ...props}) => (
  <Menu.ContextMenu id={id} className={'ui active visible dropdown'}>
    <Dropdown.Menu className={'visible'} style={{top: -70 /*needed because the context menu doesn't work properly*/}}>
      <Dropdown.Item icon='cut' text='Cut' />
      <Dropdown.Item icon='copy' text='Copy' />
      <Dropdown.Item icon='paste' text='Paste' onClick={(e) => hide()}/>
    </Dropdown.Menu>
  </Menu.ContextMenu>
)

export const ContextMenu = ({
  idx,
  id,
  rowIdx,
  ...props,
}) => {
  const colId = idx
  const rowId = rowIdx

  console.log('connect: ', Menu.connect)
  if (colId === 0) {
    return <IndexContextMenu {...props} id={id} row={rowId}/>
  } else if (rowId === 0) {
    return <ColumnContextMenu {...props} id={id} col={colId} />
  }
  return <CellContextMenu {...props} id={id} row={rowId} col={colId}/>
}

export const DefaultFormatter = (e) => {
  return e.value
}

export const NumberFormatter = (e) => {
  if (e.row[0] === '') {
    return <div style={{ textAlign: 'right' }}>{e.value}</div>
  }
  return <div style={{ textAlign: 'right' }}>{e.value}</div>
}

const RowRenderer = ({ renderBaseRow, ...props }) => {
  //if required to modify row rendering, do it here
  return renderBaseRow(props)
}

const visualToRawIndex = (index) => (index - 1)
const visualToRawColumn = (index) => (index - 1)
const rowAfter = (index) => (index + 1)
export class DataGrid extends Component {



  render() {

    let contextMenuProps = {
      onRowAdded: null,
      onRowDelete: null,
    }
    let contextMenu = null

    if (this.props.contextMenu) {
      contextMenu = this.props.contextMenu()
    }

    if (this.props.contextMenuProps) {
      contextMenuProps.onRowAdded = (rowIdx) => this.props.contextMenuProps.addRow(visualToRawIndex(rowAfter(rowIdx)))
      contextMenuProps.onRowDelete = (rowIdx) => this.props.contextMenuProps.deleteRow(visualToRawIndex(rowIdx))

      if (this.props.contextMenu) {
        contextMenu = this.props.contextMenu(contextMenuProps)
      }
    }

    let onGridRowsUpdated = null
    if (this.props.modifyValue) {
      onGridRowsUpdated = (data) => {
        console.log('onGridRowsUpdated: ', data)
        let {fromRow, updated} = data
        let [colIdx, value] = Object.entries(updated)[0] //FIXME: assuming length === 0
        this.props.modifyValue(
          visualToRawIndex(fromRow),
          visualToRawColumn(colIdx),
          value,
        )}
    }


    let cellRangeSelection = {
      onStart: args => console.log(args),
      onUpdate: args => console.log(args),
      onComplete: args => console.log(args)
    }

    let dataLength = this.props.data.length

    return (
      <>
        <style>
          {`
            .react-grid-Cell--frozen.rdg-last--frozen,
            .react-grid-Viewport .react-grid-Row:first-child > .react-grid-Cell {
              background: #1b1c1d !important;
              color: rgba(255,255,255,.7)!important;
              border: 0 !important;
              font-weight: bold;
            }

            .react-grid-Header {
              display: none !important;
            }

            .react-grid-Viewport {
              top: 0 !important;
            }
          `}
        </style>
        <Segment style={{ margin: 0, padding: 0, }}>
          <InnerDataGrid
            columns={this.props.columns}
            rowGetter={i => this.props.data[i]}
            rowsCount={dataLength}
            minHeight={700}
            onGridRowsUpdated={onGridRowsUpdated}
            rowRenderer={RowRenderer}
            contextMenu={contextMenu}
            RowsContainer={Menu.ContextMenuTrigger}
            cellRangeSelection={cellRangeSelection}
            enableCellSelect={true /*TODO: what is the tiny blue button about*/}
          />
        </Segment>
      </>
    )
  }
}
