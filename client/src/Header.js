
import React, { Component } from 'react'
import { Button, Icon, Image, Menu, Search, Segment, Sidebar } from 'semantic-ui-react'

class Header extends Component {

  state = {
    headerItem: 'home',
  }

  setHeaderItem(headerItem) {
    this.setState({
      ...this.state,
      headerItem,
    })
  }

  render() {
    let switchCompression = this.props.switchCompression

    return (
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
              onClick={(e, {name}) => switchCompression()}
            >
            {this.props.compress ?
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