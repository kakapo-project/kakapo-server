import React, { Component } from 'react'
import { Button, Header, Icon, Image, Menu, Search, Segment, Sidebar } from 'semantic-ui-react'

import Login from './Login.js'

import Tables from './Tables.js'
import Queries from './Queries.js'

const Selections = Object.freeze({
  tables: 0,
  views: 1,
  queries: 2,
  scripts: 3,
  settings: 4,
})

class App extends Component {

  state = {
    headerItem: 'home',
    selection: 0,
    compress: false,
  }

  setHeaderItem(headerItem) {
    this.setState({
      ...this.state,
      headerItem,
    })
  }

  setSelection(selection) {
    this.setState({
      ...this.state,
      selection,
    })
  }

  switchCompression() {
    this.setState({
      ...this.state,
      compress: !this.state.compress,
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
        return <Queries />
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
      <main>
        <Segment inverted attached='top' basic style={{border: 0}}>
          <Menu inverted pointing secondary>
            <Menu.Item
              name='home'
              active={this.state.headerItem === 'home'}
              onClick={(e, {name}) => this.setHeaderItem(name)} />
            <Menu.Item
              name='messages'
              active={this.state.headerItem === 'messages'}
              onClick={(e, {name}) => this.setHeaderItem(name)}
            />
            <Menu.Item
              name='friends'
              active={this.state.headerItem === 'friends'}
              onClick={(e, {name}) => this.setHeaderItem(name)}
            />
            <Menu.Menu position='right'>
              <Search
                  loading={false}
                  onResultSelect={e => {}}
                  onSearchChange={e => {}}
                />
              <Menu.Item
                name='compress'
                onClick={(e, {name}) => this.switchCompression()}
              >
              {this.state.compress ?
                <Icon name='expand' /> :
                <Icon name='compress' />
              }
              </Menu.Item>
              <Menu.Item
                name='documentation'
                onClick={(e, {name}) => {}}
              >
              <Icon name='file alternate' />
              </Menu.Item>

            </Menu.Menu>
          </Menu>
        </Segment>
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5em)'}}>
          <Sidebar
            as={Menu}
            animation='push scale down'
            icon='labeled'
            inverted
            onHide={this.handleSidebarHide}
            vertical
            visible={!this.state.compress}
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
                style={{marginTop: '30vh'}}
                onClick={e => this.setSelection(Selections.settings)}>
              <Icon name='setting' />
              Settings
            </Menu.Item>
          </Sidebar>

          <Sidebar.Pusher>
            { this.renderSelection() }
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </main>
    )
  }
}

export default App;
