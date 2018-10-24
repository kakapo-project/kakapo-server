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

import 'handsontable/dist/handsontable.full.css'

import { HotTable } from '@handsontable/react'

import Header from '../Header.js'

class Tables extends Component {

  //TODO: Filters....

  state = {
    sidebarOpen: false
  }

  constructor(props) {
    super(props);
    this.data = [
      ["", "Ford", "Volvo", "Toyota", "Honda"],
      ["2016", 10, 11, 12, 13],
      ["2017", 20, 11, 14, 13],
      ["2018", 30, 15, 12, 13]
    ];
    this.rowHeaders = [
      2016, 2017, 2018, 2019
    ]
  }

  toggleSidebar() {
    this.setState({
      ...this.state,
      sidebarOpen: !this.state.sidebarOpen,
    })
  }

  render() {
    return (
      <div>
        <Header
          editor
          sidebarOpen={this.state.sidebarOpen}
          onToggle={() => this.toggleSidebar()}
        />
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)'}}>
          <Sidebar
            as={Menu}
            animation='push overlay'
            icon='labeled'
            inverted
            direction='right'
            vertical
            visible={this.state.sidebarOpen}
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

          <Sidebar.Pusher>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded style={{ height: '100%' }}>
                <HotTable data={this.data} colHeaders={true} rowHeaders={this.rowHeaders} width="600" height="300" stretchH="all" />
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    );
  }
}

export default Tables