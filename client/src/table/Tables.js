import React, { Component } from 'react'
import {
  Button,
  Card,
  Container,
  Divider,
  Grid,
  Icon,
  Image,
  Input,
  Label,
  Menu,
  Pagination,
  Segment,
  Sidebar,
  Table } from 'semantic-ui-react'


import GridLayout from './GridLayout.js'

import Header from '../Header.js'

import { WS_URL } from '../config'

const TableSidebase = (props) => (
  <Sidebar
    as={Menu}
    animation='push overlay'
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

  //TODO: Filters....

  state = {
    sidebarOpen: false,
    data: null,
  }


  toggleSidebar() {
    this.setState({
      ...this.state,
      sidebarOpen: !this.state.sidebarOpen,
    })
  }

  componentDidMount() {
    const { name } = this.props.match.params
    const url = `${WS_URL}/table/${name}`
    let socket = new WebSocket(url);

    let sendData = {
      action: 'getTableData',
      begin: 0,
      chunkSize: 100
    }
    socket.onopen = (event) => {
      socket.send(JSON.stringify(sendData));
    }

    socket.onmessage = (event) => {
      console.log('got Some data')
      console.log(event.data);
    }

  }

  render() {
    const { name } = this.props.match.params
    console.log('table name: ', name)
    return (
      <div>
        <Header
          editor
          sidebarOpen={this.state.sidebarOpen}
          onToggle={() => this.toggleSidebar()}
        />
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)'}}>
          <TableSidebase visible={this.state.sidebarOpen} />

          <Sidebar.Pusher>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded style={{ height: '100%', overflowY: 'scroll', overflowX: 'hidden'}}>
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
                <GridLayout data={this.state.data}/>
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    );
  }
}

export default Tables