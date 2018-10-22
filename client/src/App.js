import React, { Component } from 'react'
import { Button, Header, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

import Login from './Login.js'

import Tables from './Tables.js'

const Selections = Object.freeze({
  tables: 0,
  views: 1,
  queries: 2,
  scripts: 3,
  settings: 4,
})

class App extends Component {

  state = {
    selection: 0
  }

  setSelection(selection) {
    this.setState({
      ...this.state,
      selection,
    })
  }

  renderSelection() {
    let selection = this.state.selection;

    switch (selection) {
      case Selections.tables:
        return <Tables />
      case Selections.views:
        return <Segment basic>
          <Header as='h3'>View</Header>
        </Segment>
      case Selections.queries:
        return <Segment basic>
          <Header as='h3'>Queries</Header>
        </Segment>
      case Selections.scripts:
        return <Segment basic>
          <Header as='h3'>Scripts</Header>
        </Segment>
      case Selections.settings:
        return <Segment basic>
          <Header as='h3'>Settings</Header>
        </Segment>
    }
  }

  render() {
    return (
      <Sidebar.Pushable as={Segment} style={{ height: '100vh' }}>
        <Sidebar
          as={Menu}
          animation='push scale down'
          icon='labeled'
          inverted
          onHide={this.handleSidebarHide}
          vertical
          visible={true}
          width='thin'
        >
          <Menu.Item
              as='a'
              style={{marginTop: '4vh'}}
              onClick={e => this.setSelection(Selections.tables)}>
            <Icon name='database' />
            Tables
          </Menu.Item>
          <Menu.Item
              as='a'
              onClick={e => this.setSelection(Selections.views)}>
            <Icon name='eye' />
            Views
          </Menu.Item>
          <Menu.Item
              as='a'
              onClick={e => this.setSelection(Selections.queries)}>
            <Icon name='find' />
            Queries
          </Menu.Item>
          <Menu.Item
              as='a'
              onClick={e => this.setSelection(Selections.scripts)}>
            <Icon name='code' />
            Scripts
          </Menu.Item>
          <Menu.Item
              as='a'
              style={{marginTop: '40vh'}}
              onClick={e => this.setSelection(Selections.settings)}>
            <Icon name='setting' />
            Settings
          </Menu.Item>
        </Sidebar>

        <Sidebar.Pusher>
          { this.renderSelection() }
        </Sidebar.Pusher>
      </Sidebar.Pushable>
    );
  }
}

export default App;
