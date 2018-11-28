

import React, { Component } from 'react'
import { Button, Card, Container, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import { Link } from 'react-router-dom'

import ErrorMsg from '../ErrorMsg'
import { CreateEntities } from './CreateEntities'

import { API_URL } from '../config'
class Entities extends Component {

  state = {
    tables: [],
    queries: [],
    error: null,
  }

  getEntities() {
    return this.state.tables
      .concat(this.state.queries)
  }

  setTables(entities) {
    this.setState({ tables: entities })
  }

  setQueries(entities) {
    this.setState({ queries: entities })
  }

  raiseError(msg) {
    this.setState({ error: msg })
  }

  clearError() {
    this.pullData()
    this.setState({ error: null })
  }

  pullData() {
    //tables
    fetch(`${API_URL}/manage/table`)
    .then(response => {
      return response.json()
    })
    .then(data => {
      let entities = data.map(x => ({
        name: x.name,
        type: 'table',
        icon: 'database',
        lastUpdated: 'yesterday',
        description: x.description,
        isBookmarked: false,
      }))

      this.setTables(entities)
    })
    .catch(err => {
      console.log('err: ', err.message)
      this.raiseError(err.message)
    })

    //queries
    fetch(`${API_URL}/manage/query`)
    .then(response => {
      return response.json()
    })
    .then(data => {
      let entities = data.map(x => ({
        name: x.name,
        type: 'query',
        icon: 'search',
        lastUpdated: 'yesterday',
        description: x.description,
        isBookmarked: false,
      }))

      this.setQueries(entities)
    })
    .catch(err => {
      console.log('err: ', err.message)
      this.raiseError(err.message)
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
        return `/queries/${entity.name}`
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

        <ErrorMsg error={this.state.error} onClose={() => this.clearError()} types={['Retry']} />
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