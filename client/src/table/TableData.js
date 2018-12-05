
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


const ContextMenu = ({
  idx,
  id,
  rowIdx,
  onRowDelete,
  onRowInsertAbove,
  onRowInsertBelow
}) => {
  return (
    <Menu.ContextMenu id={id} className={'ui active visible dropdown'}>
      <Dropdown.Menu className={'visible'} style={{top: -70 /*needed because the context menu doesn't work properly*/}}>
        <Dropdown.Item text='New' />
        <Dropdown.Item text='Open...' description='ctrl + o' />
        <Dropdown.Item text='Save as...' description='ctrl + s' />
        <Dropdown.Item text='Rename' description='ctrl + r' />
        <Dropdown.Item text='Make a copy' />
        <Dropdown.Item icon='folder' text='Move to folder' />
        <Dropdown.Item icon='trash' text='Move to trash' />
        <Dropdown.Divider />
        <Dropdown.Item text='Download As...' />
        <Dropdown.Item text='Publish To Web' />
        <Dropdown.Item text='E-mail Collaborators' />
      </Dropdown.Menu>
    </Menu.ContextMenu>
  );
}

class TableData extends Component {

  componentDidMount() {
    this.props.requestingTableData()
  }

  render() {

    const columns = this.props.columns.map((x, idx) => ({
      key: idx,
      name: x,
      editable: true,
    }))

    return (
      <ReactDataGrid
        columns={columns}
        rowGetter={i => this.props.data[i]}
        rowsCount={this.props.data.length}
        minHeight={500}
        onGridRowsUpdated={(e, data) => console.log('e: ', e, data)}
        contextMenu={
          <ContextMenu
            onRowDelete={(e, data) => console.log('e1: ', e, data)}
            onRowInsertAbove={(e, data) => console.log('e2: ', e, data)}
            onRowInsertBelow={(e, data) => console.log('e3: ', e, data)}
          />
        }
        RowsContainer={Menu.ContextMenuTrigger}
        enableCellSelect={true /*TODO: what is the tiny blue button about*/}
      />
    )
  }
}

const mapStateToProps = (state) => ({
  data: state.table.data,
  columns: state.table.columns,
})

const mapDispatchToProps = (dispatch) => ({
  requestingTableData: () => dispatch(requestingTableData()),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TableData)