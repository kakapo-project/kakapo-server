

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

  renderIcon(entity) {
    switch (entity.type) {
      case 'query':
        return <Icon circular size='huge' color='scheme-green' name={entity.icon} style={{boxShadow: '0 0 0 0.1em rgba(0,83,34, 1) inset'}}/>
      case 'view':
        return <Icon circular inverted size='huge' color='grey' name={entity.icon} />
      case 'table':
        return <Icon circular inverted size='huge' color='black' name={entity.icon} />
      case 'script':
        return <Icon circular inverted size='huge' color='scheme-green' name={entity.icon} />
    }
  }

  render() {
    let selectedRenderEntities = this.props.select

    let entities = this.getEntities()

    return (
      <Segment basic>
        <Grid container doubling columns={4}>
          { entities
              .filter( entity => selectedRenderEntities.includes(entity.type))
              .map( entity =>
            <Grid.Column>
                <Card>
                  <Segment textAlign='center' basic>{this.renderIcon(entity)}</Segment>
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