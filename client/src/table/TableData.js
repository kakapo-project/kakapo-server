
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


import GridLayout from './GridLayout.js'

import Header from '../Header.js'
import ErrorMsg from '../ErrorMsg'


import { WS_URL } from '../config'
import { connect } from 'react-redux'

import { requestingTableData } from '../actions'

import ReactDataGrid from 'react-data-grid'
import { Menu } from 'react-data-grid-addons'

import { hide } from './contextMenuHelper'

const ColumnContextMenu = ({id, col}) => (
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

const IndexContextMenu = ({id, row}) => (
  <Menu.ContextMenu id={id} className={'ui active visible dropdown'}>
    <Dropdown.Menu className={'visible'} style={{top: -70 /*needed because the context menu doesn't work properly*/}}>
      <Dropdown.Item icon='add' text='Add Row' />
      <Dropdown.Item icon='clone' text='Duplicate Row' />
      <Dropdown.Item icon='trash' text='Delete Row' />
      <Dropdown.Divider />
      <Dropdown.Item icon='cut' text='Cut' />
      <Dropdown.Item icon='copy' text='Copy' />
      <Dropdown.Item icon='paste' text='Paste' onClick={(e) => hide()}/>
    </Dropdown.Menu>
  </Menu.ContextMenu>
)

const CellContextMenu = ({id, col, row}) => (
  <Menu.ContextMenu id={id} className={'ui active visible dropdown'}>
    <Dropdown.Menu className={'visible'} style={{top: -70 /*needed because the context menu doesn't work properly*/}}>
      <Dropdown.Item icon='cut' text='Cut' />
      <Dropdown.Item icon='copy' text='Copy' />
      <Dropdown.Item icon='paste' text='Paste' onClick={(e) => hide()}/>
    </Dropdown.Menu>
  </Menu.ContextMenu>
)

const ContextMenu = ({
  idx,
  id,
  rowIdx,
  onRowDelete,
  onRowInsertAbove,
  onRowInsertBelow
}) => {
  const colId = idx
  const rowId = rowIdx

  console.log('connect: ', Menu.connect)
  if (colId === 0) {
    return <IndexContextMenu id={id} row={rowId}/>
  } else if (rowId === 0) {
    return <ColumnContextMenu id={id} col={colId} />
  }
  return <CellContextMenu id={id} row={rowId} col={colId}/>
}

const DefaultFormatter = (e) => {
  return e.value
}

const NumberFormatter = (e) => {
  if (e.row[0] === '') {
    return <div style={{ textAlign: 'right' }}>{e.value}</div>
  }
  return <div style={{ textAlign: 'right' }}>{e.value}</div>
}

const RowRenderer = ({ renderBaseRow, ...props }) => {
  //if required to modify row rendering, do it here
  return renderBaseRow(props)
}


class TableData extends Component {

  componentDidMount() {
    this.props.requestingTableData()
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
          <ReactDataGrid
            columns={columns}
            rowGetter={i => data[i]}
            rowsCount={data.length}
            minHeight={500}
            onGridRowsUpdated={(e, data) => console.log('e: ', e, data)}
            rowRenderer={RowRenderer}
            contextMenu={
              <ContextMenu
                onRowDelete={(e, data) => console.log('e1: ', e, data)}
                onRowInsertAbove={(e, data) => console.log('e2: ', e, data)}
                onRowInsertBelow={(e, data) => console.log('e3: ', e, data)}
              />
            }
            RowsContainer={Menu.ContextMenuTrigger}
            cellRangeSelection={{
              onStart: args => console.log(args),
              onUpdate: args => console.log(args),
              onComplete: args => console.log(args)
            }}
            enableCellSelect={true /*TODO: what is the tiny blue button about*/}
          />
        </Segment>
      </>
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
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TableData)