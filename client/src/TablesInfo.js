

import React, { Component } from 'react'
import { Button, Card, Container, Header, Grid, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

class TableInfo extends Component {

  getTables() {
    return [
      {
        name: 'characters',
        icon: 'users',
        lastUpdated: 'yesterday',
        description: 'The list of characters in our game',
        isBookmarked: true,
      },
      {
        name: 'weapons',
        icon: 'bomb',
        lastUpdated: 'yesterday',
        description: 'The list of weapons in our game',
        isBookmarked: false,
      },
      {
        name: 'quests',
        icon: 'exclamation',
        lastUpdated: 'yesterday',
        description: 'The list of quests',
        isBookmarked: false,
      },
      {
        name: 'npcs',
        icon: 'male',
        lastUpdated: 'yesterday',
        description: 'Users',
        isBookmarked: false,
      },
      {
        name: 'guilds',
        icon: 'shield',
        lastUpdated: 'yesterday',
        description: 'Guilds that the users belongs to',
        isBookmarked: false,
      },
    ]
  }

  render() {
    let tables = this.getTables()

    return (
      <Grid container doubling columns={4}>
        { tables.map( table =>
          <Grid.Column>
              <Card>
                <Segment textAlign='center' basic>
                  <Icon circular inverted size='huge' color='black' name={table.icon} />
                </Segment>
                <Card.Content>
                  <Card.Header>{table.name}</Card.Header>
                  <Card.Meta>last updated {table.lastUpdated}</Card.Meta>
                  <Card.Description>{table.description}</Card.Description>
                </Card.Content>
                <Card.Content extra>
                  <a>
                    <Icon name='favorite' color={(table.isBookmarked)? 'yellow': 'grey'}/>
                    Bookmark
                  </a>
                </Card.Content>
              </Card>
          </Grid.Column>
        )}
      </Grid>
    )
  }
}

export default TableInfo