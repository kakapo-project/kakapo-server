import React, { Component } from 'react'
import Tab, { Button, Icon, Image, Menu, Search, Segment, Sidebar } from 'semantic-ui-react'

import Login from './Login.js'

import Header from './Header.js'
import Entities from './Entities.js'
import Settings from './Settings.js'

const Tabs = Object.freeze({
  entities: 0,
  settings: 1,
})

const Selections = Object.freeze({
  tables: 'table',
  queries: 'query',
  views: 'view',
  scripts: 'script',
})

class Home extends Component {

  state = {
    tab: 0,
    selections: [Selections.tables],
    compress: false,
  }

  setTab(tab) {
    this.setState({
      ...this.state,
      tab,
    })
  }

  setEntitySelection(selection) {

    let newSelections = [...this.state.selections]
    if (newSelections.includes(selection)) {
      newSelections = newSelections.filter(x => x != selection)
    } else {
      newSelections = newSelections.concat([selection])
    }

    console.log('selection: ', newSelections)
    this.setState({
      ...this.state,
      tab: Tabs.entities,
      selections: newSelections
    })
  }

  toggleSidebar() {
    this.setState({
      ...this.state,
      sidebarOpen: !this.state.sidebarOpen,
    })
  }

  renderSelection() {
    let tab = this.state.tab;

    switch (tab) {
      case Tabs.entities:
        return <Entities select={this.state.selections} />
      case Tabs.settings:
        return <Settings />
    }
  }

  render() {
    console.log('this.state: ', this.state)
    return (
      <div>
        <Header
          sidebarOpen={this.state.sidebarOpen}
          onToggle={() => this.toggleSidebar()}
        />
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5em)'}}>
          <style>
            {`
              .ui.inverted.menu .dark-green.active.item, .ui.inverted.dark-green.menu {
                background-color: #005322;
              }
              .ui.menu .light-green.active.item, .ui.light-green.menu {
                background-color: #C9E400;
              }
            `}
          </style>
          <Sidebar
            as={Menu}
            animation='push scale down'
            icon='labeled'
            inverted
            color='black'
            vertical
            visible={!this.state.sidebarOpen}
            width='thin'
          >
            <Menu.Item
                as='a'
                active={this.state.selections.includes(Selections.tables)}
                style={{marginTop: '4vh'}}
                onClick={e => this.setEntitySelection(Selections.tables)}>
              <Icon name='database' />
              Tables
            </Menu.Item>
            <Menu.Item
                as='a'
                active={this.state.selections.includes(Selections.views)}
                onClick={e => this.setEntitySelection(Selections.views)}>
              <Icon name='eye' />
              Views
            </Menu.Item>
            <Menu.Item
                as='a'
                active={this.state.selections.includes(Selections.queries)}
                onClick={e => this.setEntitySelection(Selections.queries)}>
              <Icon name='find' />
              Queries
            </Menu.Item>
            <Menu.Item
                as='a'
                active={this.state.selections.includes(Selections.scripts)}
                onClick={e => this.setEntitySelection(Selections.scripts)}>
              <Icon name='code' />
              Scripts
            </Menu.Item>
            <Menu.Item
                as='a'
                active={this.state.tab == Tabs.settings}
                style={{marginTop: '30vh'}}
                onClick={e => this.setTab(Tabs.settings)}>
              <Icon name='setting' />
              Settings
            </Menu.Item>
          </Sidebar>

          <Sidebar.Pusher>
            { this.renderSelection() }
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    )
  }
}

export default Home;
