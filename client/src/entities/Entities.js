

import React, { Component } from 'react'
import { Button, Card, Container, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import { Link } from 'react-router-dom'

import ErrorMsg from '../ErrorMsg'
import CreateEntities from './CreateEntities'

import { API_URL } from '../config'

import { pullData, clearPullDataError } from '../actions'
import { connect } from 'react-redux'
class Entities extends Component {

  getEntities() {
    return this.props.data.tables
      .concat(this.props.data.queries)
      .concat(this.props.data.script)
  }

  clearError() {
    this.props.pullData()
    this.props.clearError()
  }


  componentDidMount() {
    this.props.pullData()
  }

  renderIcon(entity) {
    switch (entity.type) {
      case 'query':
        return <Icon circular size='huge' className='scheme-green' name={entity.icon} style={{boxShadow: '0 0 0 0.1em rgba(0,83,34, 1) inset'}}/>
      case 'view':
        return <Icon circular inverted size='huge' color='grey' name={entity.icon} />
      case 'table':
        return <Icon circular inverted size='huge' color='black' name={entity.icon} />
      case 'script':
        return <Icon circular inverted size='huge' className='scheme-green' name={entity.icon} />
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
        return `/scripts/${entity.name}`
    }
  }

  render() {
    let selectedRenderEntities = this.props.select

    let entities = this.getEntities()

    if (this.props.isDirty) {
      this.props.pullData()
    }

    return (
      <Segment basic>

        <ErrorMsg error={this.props.error} onClose={() => this.clearError()} types={['Retry']} />
        <CreateEntities />

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


const mapStateToProps = (state) => ({
  data: state.data,
  isDirty: state.entityCreator.entitiesDirty,
  error: state.data.error,
})

const mapDispatchToProps = (dispatch) => ({
  pullData: () => dispatch(pullData()),
  clearError: () => dispatch(clearPullDataError()),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Entities)
