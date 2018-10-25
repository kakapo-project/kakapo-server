
import React, { Component } from 'react'
import { Button, Card, Container, Divider, Form, Grid, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

import CodeMirror from 'react-codemirror'


import 'codemirror/addon/hint/sql-hint'
import 'codemirror/lib/codemirror.css'
import 'codemirror/theme/darcula.css'

import Header from '../Header.js'


class Queries extends Component {

  state = {
    sidebarOpen: false
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
              Download File
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='upload' />
              Upload File
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

          <Sidebar.Pusher>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded='very' style={{ height: '100%' }}>
                <Form>
                  <CodeMirror options={{
                    theme: 'darcula',
                    mode: 'text/x-mysql',
                    lineNumbers: true,
                    styleActiveLine: true,
                  }} />
                </Form>
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    )
  }

  // Generate Query fields
  // Run Button
  // Dry Run Button
  // NOTE: Auto run if no changes within 5 seconds
}

export default Queries