

import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Header, Grid, Icon, Image, Input, Menu, Modal, Segment, Sidebar, Transition } from 'semantic-ui-react'
import ErrorMsg from '../../ErrorMsg'

import { API_URL } from '../../config'
import { DEFAULT_TYPE, ALL_TYPES } from '../../actions/columns'

import {
  setTableName,
  addColumn,
  setPrimaryKey,
  setColumnType,
  setColumnName,
  removeColumn,
  moveColumnDown,
  moveColumnUp,
} from '../../actions/createTable'

import { connect } from 'react-redux'


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

class CreateTable extends Component {

  getColumns() {
    let columns = { ...this.props.columns }
    let columnKeys = getAllKeys(columns)
    columnKeys.sort()

    return columnKeys.map(x => ({ ...columns[x], key: x}))
  }

  render() {

    return (
      <>
        <Grid>
          <Grid.Column floated='left' width={10}>
            <Button
              positive
              onClick={e =>this.props.addColumn()}
            >Add Column</Button>
          </Grid.Column>
          <Grid.Column width={6}>
            <Input
              placeholder='Table Name'
              fluid
              value={this.props.tableName}
              onChange={(e, data) => this.props.setTableName(data.value)}
              />
          </Grid.Column>
        </Grid>
        <Divider hidden/>
        <Grid>
          {
            this
              .getColumns()
              .map((x, idx) => <ColumnItem
                key={x.key}
                column={x}
                primaryKey={this.props.primaryKey}
                setPrimaryKey={x => this.props.setPrimaryKey(x)}
                setColumnName={(x, val) => this.props.setColumnName(x, val)}
                setColumnType={(x, val) => this.props.setColumnType(x, val)}
                removeColumn={x => this.props.removeColumn(x)}
                moveDown={x => this.props.moveDown(x)}
                moveUp={x => this.props.moveUp(x)}
              />)
          }
        </Grid>
      </>
    )
  }
}


const mapStateToProps = (state) => ({
  tableName: state.entityCreator.tableName,
  columns: state.entityCreator.columns,
  primaryKey: state.entityCreator.primaryKey,
})

const mapDispatchToProps = (dispatch) => ({
  setTableName: (name) => dispatch(setTableName(name)),
  addColumn: () => dispatch(addColumn()),
  setPrimaryKey: (idx) => dispatch(setPrimaryKey(idx)),
  setColumnName: (idx, val) => dispatch(setColumnName(idx, val)),
  setColumnType: (idx, val) => dispatch(setColumnType(idx, val)),
  removeColumn: (idx) => dispatch(removeColumn(idx)),
  moveDown: (idx) => dispatch(moveColumnDown(idx)),
  moveUp: (idx) => dispatch(moveColumnUp(idx)),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(CreateTable)
