
import React, { Component } from 'react'
import { Button, Divider, Icon, Image, Menu, Search, Segment, Sidebar } from 'semantic-ui-react'
import { Link } from 'react-router-dom'
import { connect } from 'react-redux'
import { clickToggleSidebar } from './actions'

import logo from './logo.svg'
class Header extends Component {

  state = {
  }

  render() {
    let compress = this.props.isSidebarOpen()

    return (
      <Segment inverted basic style={{border: 0, height: '5em', textAlign: 'bottom', margin: 0}}>
        <Menu inverted pointing secondary style={{height: '3.1em'}}>
          <style>
            {`
              #home-button {
                margin-bottom: -1.6em;
                margin-left: 1.42em;
                margin-right: 0.2em;
                width: 6.0em;
                height: auto;
                transition: all .15s ease;
              }
              #home-button:hover {
                margin-bottom: -1.8em;
                margin-left: 1.22em;
                margin-right: 0.0em;
                width: 6.4em;
              }
            `}
          </style>
          <Menu.Item id='home-button' as={Link} to='/'>
            <Image src={logo} />
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
              onClick={(e, {name}) => this.props.clickToggleSidebar()}
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

const mapStateToProps = (state) => ({
  isSidebarOpen: () => state.sidebarOpen,
})

const mapDispatchToProps = (dispatch) => ({
  clickToggleSidebar: () => dispatch(clickToggleSidebar()),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Header)