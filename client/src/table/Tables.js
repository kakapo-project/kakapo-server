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

import { tableWantsToLoad, loadedPage } from '../actions'

import TableData from './TableData'


const TableSidebase = (props) => (
  <Sidebar
    as={Menu}
    animation='overlay'
    icon='labeled'
    inverted
    direction='right'
    vertical
    visible={props.visible}
    width='thin'
  >
    <Menu.Item
        as='a'>
      <Icon name='download' />
      Export Data
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='cloud upload' />
      Import Data
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='anchor' />
      API
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='history' />
      History
    </Menu.Item>
    <Divider />
    <Menu.Item
        as='a'>
      <Icon name='plus' />
      Create New
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='clone' />
      Duplicate
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='edit' />
      Modify
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='undo alternate' />
      Rollback
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='trash' />
      Delete
    </Menu.Item>
    <Divider />
    <Menu.Item
        as='a'>
      <Icon name='shield' />
      Access
    </Menu.Item>
  </Sidebar>
)

class Tables extends Component {


  componentWillMount() {
    this.props.loadedPage()
  }

  componentDidMount() {
    const { name } = this.props.match.params
    this.props.tableWantsToLoad(name)
  }

  render() {
    return (
      <div>
        <Header editor />
        {/* <ErrorMsg error={this.props.error} onClose={(type) => this.closeErrorMessage(type)} types={this.errorMsgTypes}/> */}
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)'}}>
          <TableSidebase visible={this.props.isSidebarOpen()} />

          <Sidebar.Pusher>
            <Dimmer active={!this.props.isTableLoaded()}>
              <Loader size='big'>Loading</Loader>
            </Dimmer>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded style={{ height: '100%', overflowX: 'hidden'}}>
                <Segment>
                  <Label as='a'>
                    <Icon name='mouse pointer' />
                    select
                    <Icon name='delete' />
                  </Label>
                  <Label as='a'>
                    <Icon name='filter' />
                    where
                    <Icon name='delete' />
                  </Label>
                  <Label as='a'>
                    <Icon name='sort' />
                    order by
                    <Icon name='delete' />
                  </Label>
                  <Label as='a' color='green'>
                    <Icon name='add' style={{marginRight: 0}}/>
                  </Label>
                </Segment>
                { this.props.isTableConnected() ? <TableData /> : <></> }
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    );
  }
}


const mapStateToProps = (state) => ({
  isTableConnected: () => state.table.isConnected,
  isTableLoaded: () => state.table.isLoaded,
  isSidebarOpen: () => state.sidebar.isOpen,
  error: null,
})

const mapDispatchToProps = (dispatch) => ({
  tableWantsToLoad: name => dispatch(tableWantsToLoad(name)),
  loadedPage: () => dispatch(loadedPage('Tables')),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Tables)