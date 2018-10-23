

import React, { Component } from 'react'
import { Button, Card, Container, Header, Grid, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

class Entities extends Component {

  getEntities() {
    return [
      {
        name: 'characters',
        type: 'table',
        icon: 'users',
        lastUpdated: 'yesterday',
        description: 'The list of characters in our game',
        isBookmarked: true,
      },
      {
        name: 'weapons',
        type: 'table',
        icon: 'bomb',
        lastUpdated: 'yesterday',
        description: 'The list of weapons in our game',
        isBookmarked: false,
      },
      {
        name: 'quests',
        type: 'table',
        icon: 'exclamation',
        lastUpdated: 'yesterday',
        description: 'The list of quests',
        isBookmarked: false,
      },
      {
        name: 'npcs',
        type: 'table',
        icon: 'male',
        lastUpdated: 'yesterday',
        description: 'Users',
        isBookmarked: false,
      },
      {
        name: 'guilds',
        type: 'table',
        icon: 'shield',
        lastUpdated: 'yesterday',
        description: 'Guilds that the users belongs to',
        isBookmarked: false,
      },
      {
        name: 'select_characters',
        type: 'query',
        icon: 'chevron down',
        lastUpdated: 'get all character',
        description: 'The list of quests',
        isBookmarked: true,
      },
      {
        name: 'insert_characters',
        type: 'query',
        icon: 'chevron up',
        lastUpdated: 'yesterday',
        description: 'Add new character',
        isBookmarked: false,
      },
      {
        name: 'run_analytics',
        type: 'script',
        icon: 'search',
        lastUpdated: 'yesterday',
        description: 'Run weekly script',
        isBookmarked: false,
      },
      {
        name: 'user_quests',
        type: 'view',
        icon: 'pointing right',
        lastUpdated: 'yesterday',
        description: 'Join user and quests',
        isBookmarked: false,
      },
    ]
  }

  render() {
    let selectedRenderEntities = this.props.select

    let entities = this.getEntities()

    let colors = {
      'query': 'grey',
      'view': 'grey',
      'table': 'black',
      'script': 'blue',
    }

    let isInverted = {
      'query': false,
      'view': true,
      'table': true,
      'script': true,
    }

    return (
      <Segment basic>
        <Grid container doubling columns={4}>
          { entities
              .filter( entity => selectedRenderEntities.includes(entity.type))
              .map( entity =>
            <Grid.Column>
                <Card>
                  <Segment textAlign='center' basic>
                    <Icon circular inverted={isInverted[entity.type]} size='huge' color={colors[entity.type]} name={entity.icon} />
                  </Segment>
                  <Card.Content>
                    <Card.Header>{entity.name}</Card.Header>
                    <Card.Meta>last updated {entity.lastUpdated}</Card.Meta>
                    <Card.Description>{entity.description}</Card.Description>
                  </Card.Content>
                  <Card.Content extra>
                    <a>
                      <Icon name='favorite' color={(entity.isBookmarked)? 'yellow': 'grey'}/>
                      Bookmark
                    </a>
                  </Card.Content>
                </Card>
            </Grid.Column>
          )}
        </Grid>
      </Segment>
    )
  }
}

export default Entities