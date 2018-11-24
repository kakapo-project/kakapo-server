

import React, { Component } from 'react'
import { Button, Card, Container, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import { Link } from 'react-router-dom'

import { CreateEntities } from './CreateEntities'

import { API_URL } from '../config'
class Entities extends Component {

  state = {
    entities: [],
  }

  getEntities() {
    return this.state.entities
  }

  setEntitites(entities) {
    this.setState({ entities: entities })
  }

  pullData() {
    fetch(`${API_URL}/manage/table`)
    .then(response => {
      return response.json()
    })
    .then(data => {
      let entities = data.map(x => ({
        name: x.name,
        type: 'table',
        icon: 'search',
        lastUpdated: 'yesterday',
        description: x.description,
        isBookmarked: false,
      }))

      this.setEntitites(entities)
    })
    .catch(err => {
      console.log('err: ', err.message)
    })
  }

  componentDidMount() {
    this.pullData()
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

  getEntityLink(entity) {
    switch (entity.type) {
      case 'query':
        return `/`
      case 'view':
        return '/'
      case 'table':
        return `/tables/${entity.name}`
      case 'script':
        return '/'
    }
  }

  render() {
    let selectedRenderEntities = this.props.select

    let entities = this.getEntities()

    return (
      <Segment basic>

        <CreateEntities onCreated={() => this.pullData()}/>

        <Transition.Group as={Grid} animation='scale' duration={400} container doubling columns={4} >
          { entities
              .filter( entity => selectedRenderEntities.includes(entity.type))
              .map( (entity, idx) =>
            <Grid.Column key={idx}>
              <Card
                link
                as={Link}
                to={this.getEntityLink(entity)}
              >
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
        </Transition.Group>
      </Segment>
    )
  }
}

export default Entities