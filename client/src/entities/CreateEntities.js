

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'

import { API_URL } from '../config'
import { DEFAULT_TYPE, ALL_TYPES } from './columns'

const ColumnItem = (props) => (
  <Grid.Row>
    <Grid.Column width={10}>
      <Input
        label={
          <Dropdown
            defaultValue={DEFAULT_TYPE}
            options={ALL_TYPES.map(x => ({key: x, text: x, value: x}))}
            onChange={(e, data) => props.setColumnType(props.column.key, data.value)}
          />}
        labelPosition='right'
        placeholder='Column Name'
        fluid
        value={(props.column && props.column.name) ? props.column.name : '' }
        onChange={(e, data) => props.setColumnName(props.column.key, data.value)}
      />
    </Grid.Column>
    <Grid.Column width={3}>
      <Button
        floated='right'
        fluid
        active={props.column.key === props.primaryKey}
        positive={props.column.key === props.primaryKey}
        onClick={e => props.setPrimaryKey(props.column.key)}
      >Primary Key</Button>
    </Grid.Column>
    <Grid.Column width={1}>
      <Button
        circular
        icon='angle up'
        onClick={e => props.moveUp(props.column.key)}
      />
    </Grid.Column>
    <Grid.Column width={1}>
      <Button
        circular
        icon='angle down'
        onClick={e => props.moveDown(props.column.key)}
      />
    </Grid.Column>
    <Grid.Column width={1}>
      <Button
        circular
        negative
        icon='delete'
        onClick={e => props.removeColumn(props.column.key)}
      />
    </Grid.Column>
  </Grid.Row>
)

const getAllKeys = (obj) => Object.keys(obj).map(x => parseInt(x))

class CreateEntities extends Component {

  initialState = {
    columns: { 0: null },
    creatingEntities: false,
    primaryKey: 0,
  }

  state = { ...this.initialState }

  handleCreatingEntities() {
    this.setState({ creatingEntities: true })
  }

  handleCreatedEntities() {
    console.log(this.state)
    this.setState({ ...this.initialState })
  }

  isCreatingEntites() {
    return this.state.creatingEntities
  }

  getPrimaryKey() {
    return this.state.primaryKey
  }

  setPrimaryKey(key) {
    this.setState({ primaryKey: key })
  }

  setColumnType(key, typeName) {
    let columns = { ...this.state.columns }
    columns[key] = { ...columns[key], typeName: typeName }
    this.setState({ columns: columns})
  }

  setColumnName(key, name) {
    let columns = { ...this.state.columns }
    columns[key] = { ...columns[key], name: name }
    this.setState({ columns: columns})
  }

  getColumns() {
    let columns = { ...this.state.columns }
    let columnKeys = getAllKeys(columns)
    columnKeys.sort()

    return columnKeys.map(x => ({ ...columns[x], key: x}))
  }

  moveUp(key) {
    let newColumns = { ...this.state.columns }
    let columnKeys = getAllKeys(newColumns)
    columnKeys.sort()

    let columnKeyIndex = columnKeys.indexOf(key)
    if (columnKeyIndex === 0) {
      return
    }

    let temp = newColumns[columnKeys[columnKeyIndex - 1]]
    newColumns[columnKeys[columnKeyIndex - 1]] = newColumns[columnKeys[columnKeyIndex]]
    newColumns[columnKeys[columnKeyIndex]] = temp

    // for the primary keys
    let newPrimaryKey = this.state.primaryKey
    if (columnKeys[columnKeyIndex] === this.state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex - 1]
    } else if (columnKeys[columnKeyIndex - 1] === this.state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex]
    }

    this.setState({ columns: newColumns, primaryKey: newPrimaryKey })
  }

  moveDown(key) {
    let newColumns = { ...this.state.columns }
    let columnKeys = getAllKeys(newColumns)
    columnKeys.sort()

    let columnKeyIndex = columnKeys.indexOf(key)
    if (columnKeyIndex === columnKeys.length - 1) {
      return
    }

    let temp = newColumns[columnKeys[columnKeyIndex + 1]]
    newColumns[columnKeys[columnKeyIndex + 1]] = newColumns[columnKeys[columnKeyIndex]]
    newColumns[columnKeys[columnKeyIndex]] = temp

    // for the primary keys
    let newPrimaryKey = this.state.primaryKey
    if (columnKeys[columnKeyIndex] === this.state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex + 1]
    } else if (columnKeys[columnKeyIndex + 1] === this.state.primaryKey) {
      newPrimaryKey = columnKeys[columnKeyIndex]
    }

    this.setState({ columns: newColumns, primaryKey: newPrimaryKey })

  }

  addColumn() {
    let lastKey = Math.max(...getAllKeys(this.state.columns))
    let columns = {
      ...this.state.columns,
      [lastKey+1]: null
    }
    this.setState({ columns: columns })
  }

  removeColumn(key) {
    let columns = { ...this.state.columns }
    delete columns[key]

    //handle primary key
    let primaryKey = this.state.primaryKey
    console.log('A: ', key, ' B:', this.state.primaryKey)
    if (key === this.state.primaryKey) {
      let allKeys = getAllKeys(columns)

      if (allKeys.length === 0) {  //handle remove all case
        columns = { 0: null }
        primaryKey = 0
      } else {
        primaryKey = allKeys[0]
      }
    }

    this.setState({ columns: columns, primaryKey: primaryKey })
  }

  render() {

    return (
      <Modal
        trigger={<Button
          circular
          positive
          icon='plus'
          floated='right'
          size='massive'
          onClick={e =>this.handleCreatingEntities()}
        />}
        open={this.isCreatingEntites()}
        onClose={e => this.handleCreatedEntities()}
        basic
      >
        <Header icon='database' content='Create New Table' />
        <Modal.Content>
          <Button
            positive
            onClick={e =>this.addColumn()}
          >Add Column</Button>
          <Divider hidden/>
          <Grid>
            {
              this
                .getColumns()
                .map((x, idx) => <ColumnItem
                  key={x.key}
                  column={x}
                  primaryKey={this.getPrimaryKey()}
                  setPrimaryKey={x => this.setPrimaryKey(x)}
                  setColumnName={(x, val) => this.setColumnName(x, val)}
                  setColumnType={(x, val) => this.setColumnType(x, val)}
                  removeColumn={x => this.removeColumn(x)}
                  moveDown={x => this.moveDown(x)}
                  moveUp={x => this.moveUp(x)}
                />)
            }
          </Grid>


        </Modal.Content>
        <Modal.Actions>
          <Button color='green' onClick={e => this.handleCreatedEntities()} inverted>
            <Icon name='checkmark' />Create
          </Button>
        </Modal.Actions>
      </Modal>
    )
  }
}

export { CreateEntities }