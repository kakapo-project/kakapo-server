

import React, { Component } from 'react'
import { Button, Card, Container, Header, Grid, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

import TablesData from './TablesData.js'
import TablesInfo from './TablesInfo.js'

class Tables extends Component {

  state = {
    showInfo: false
  }

  render() {
    return (
      <Segment basic style={{height: '100vh', overflowY: 'scroll'}}>
        { this.state.showInfo ?
          <TablesInfo /> :
          <TablesData />
        }
      </Segment>
    )
  }
}

export default Tables