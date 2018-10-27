
import React, { Component } from 'react'
import { Button, Divider, Icon, Image, Menu, Search, Segment, Sidebar } from 'semantic-ui-react'

import logo from './logo.svg'
class Header extends Component {

  state = {
  }

  render() {
    let switchCompression = this.props.onToggle
    let compress = this.props.sidebarOpen

    return (
      <Segment inverted attached='top' basic style={{border: 0, height: '5em', textAlign: 'bottom'}}>
        <Menu inverted pointing secondary style={{height: '3.1em'}}>
          <Menu.Item as='a' style={{marginBottom: '-1.35em', marginLeft: '1.65em'}}>
            <Image src={logo} style={{width: 45, height: 'auto'}} />
          </Menu.Item>
          { this.props.editor && (
            <Menu secondary>
              <Menu.Item
                as='a'
              >
                <Icon name='undo' />
              </Menu.Item>
              <Menu.Item
                as='a'
              >
                <Icon name='redo' />
              </Menu.Item>
              <Divider />
              <Menu.Item
                as='a'
              >
                <Icon name='cut' />
              </Menu.Item>
              <Menu.Item
                as='a'
              >
                <Icon name='copy' />
              </Menu.Item>
              <Menu.Item
                as='a'
              >
                <Icon name='paste' />
              </Menu.Item>
            </Menu>
          )}
          <Menu.Menu position='right'>
            <Search
                loading={false}
                onResultSelect={e => {}}
                onSearchChange={e => {}}
              />
            <Menu.Item
              name='compress'
              onClick={(e, {name}) => switchCompression()}
            >
              {compress ?
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
    )
  }
}

export default Header