import React, { Component } from 'react'
import {
  Button,
  Card,
  Container,
  Divider,
  Grid,
  Icon,
  Image,
  Input,
  Label,
  Menu,
  Pagination,
  Segment,
  Sidebar,
  Table } from 'semantic-ui-react'


import GridLayout from './GridLayout.js'

import Header from '../Header.js'

class Tables extends Component {

  //TODO: Filters....

  state = {
    sidebarOpen: false
  }



  toggleSidebar() {
    this.setState({
      ...this.state,
      sidebarOpen: !this.state.sidebarOpen,
    })
  }

  renderTypeSymbol(type) {
    switch (type) {
      case 'Boolean':
        return '<i aria-hidden="true" class="check icon">'
      case 'String':
        return '<i aria-hidden="true" class="font icon">'
      case 'Integer':
        return '<i aria-hidden="true" class="hashtag icon">'
      case 'Number':
        return '<i aria-hidden="true" class="times icon">'
      case 'Percentage':
        return '<i aria-hidden="true" class="percent icon">'
      case 'Money':
        return '<i aria-hidden="true" class="dollar icon">'
      case 'Date':
        return '<i aria-hidden="true" class="calendar icon">'
      case 'DateTime':
        return '<i aria-hidden="true" class="clock icon">'
      case 'Json':
        return '{}'
      default:
        return ''
    }
  }

  getHandsontableType(type) {

    switch (type) {
      case 'Boolean':
        return 'checkbox'
      default:
        return 'text'
    }
  }


  renderColumnHeader(column) {

    const renderKeySymbol = (column) => {
      if (column.isPrimaryKey) {
        return '  <i aria-hidden="true" class="key icon">'
      } else if (column.isForeignKey) {
        return '  <i aria-hidden="true" class="linkify icon">'
      } else {
        return ''
      }
    }

    return `<strong>${column.name}</strong>${renderKeySymbol(column)}`
  }

  renderIndexForRowWithNoKey() {
    return '<i aria-hidden="true" class="question icon">'
  }

  getData() {
    let schema = {
      columns: [
        {
          name: 'id',
          dataType: 'Integer'
        },
        {
          name: 'name',
          dataType: 'String'
        },
        {
          name: 'age',
          dataType: 'Integer'
        },
        {
          name: 'data',
          dataType: 'Json'
        },
        {
          name: 'last_visited',
          dataType: 'DateTime'
        },
        {
          name: 'joined',
          dataType: 'Date'
        },
        {
          name: 'is_admin',
          dataType: 'Boolean'
        },
      ],
      constraints: [
        {
          key: 'id'
        },
        {
          reference: {
            column: 'age',
            foreignTable: 'other_table',
            foreignColumn: 'other_table_id',
          },
        },
      ]
    }

    let data = [
      {
        'id': 1,
        'name': 'I.P. Freely',
        'age': 69,
        'data': '{}',
        'last_visited': new Date(),
        'joined': new Date(),
        'is_admin': false,
      },
      {
        'id': 2,
        'name': 'I.P. Freely',
        'age': 69,
        'data': '{}',
        'last_visited': new Date(),
        'joined': new Date(),
        'is_admin': true,
      },
      {
        'id': 3,
        'name': 'I.P. Freely',
        'age': 69,
        'data': '{}',
        'last_visited': new Date(),
        'joined': new Date(),
        'is_admin': false,
      },
      {
        'id': 4,
        'name': 'I.P. Freely',
        'age': 69,
        'data': '{}',
        'last_visited': new Date(),
        'joined': new Date(),
        'is_admin': true,
      },
      {
        'id': 5,
        'name': 'I.P. Freely',
        'age': 69,
        'data': '{}',
        'last_visited': new Date(),
        'joined': new Date(),
        'is_admin': false,
      },
      {
        'id': 6,
        'name': 'I.P. Freely',
        'age': 69,
        'data': '{}',
        'last_visited': new Date(),
        'joined': new Date(),
        'is_admin': true,
      },
    ]

    return { schema, data }
  }

  getColumnMetadata(column) {
    return {
      data: column.name,
      type: this.getHandsontableType(column.dataType),
    }
  }

  getColumnsWithKey() {
    let { schema } = this.getData()
    let { columns, constraints } = schema

    let keyConstraints = constraints.map(x => x.key).filter(x => x !== undefined)
    let key = null
    if (keyConstraints.length !== 0) {
      key = keyConstraints[0]
    } else if (keyConstraints.length > 1) {
      console.log('warning, more than one primary key found. Server is wrong')
    }

    let foreignKeyConstraints = constraints.map(x => x.reference).filter(x => x !== undefined)
    let foreignKeys = foreignKeyConstraints.map(x => x.column)

    return { columns, key, foreignKeys }
  }


  getColumns() {
    let { columns, key, foreignKeys } = this.getColumnsWithKey()

    let columnsByName = {}
    for (let column of columns) {
      columnsByName[column.name] = {...column, isPrimaryKey: false, isForeignKey: false}
    }

    // add in the primary key
    if (key in columnsByName) {
      columnsByName[key] = {...columnsByName[key], isPrimaryKey: true}
    } else {
      console.log('warning, could not find key in any of the columns. Server is wrong')
    }

    // add in the foreign keys
    for (let key of foreignKeys) {
      if (key in columnsByName) {
        columnsByName[key] = {...columnsByName[key], isForeignKey: true}
      } else {
        console.log('warning, could not find key in any of the columns. Server is wrong')
      }
    }

    return Object.values(columnsByName)
  }


  getIndices() {

    let { key } = this.getColumnsWithKey()

    let { data } = this.getData()

    let indices
    if (key !== null) {
      indices = data.map(x => x[key])
    } else {
      indices = data.map(x => this.renderIndexForRowWithNoKey())
    }

    return indices
  }

  getRows() {
    let { columns } = this.getColumnsWithKey()

    let { data } = this.getData()

    //const orderBasedOnColumn = (row) => columns.map(column => row[column.name])

    return data //data.map(row => orderBasedOnColumn(row))
  }

  render() {
    return (
      <div>
        <Header
          editor
          sidebarOpen={this.state.sidebarOpen}
          onToggle={() => this.toggleSidebar()}
        />
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)'}}>
          <Sidebar
            as={Menu}
            animation='push overlay'
            icon='labeled'
            inverted
            direction='right'
            vertical
            visible={this.state.sidebarOpen}
            width='thin'
          >
            <Menu.Item
                as='a'>
              <Icon name='download' />
              Export Data
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='cloud upload' />
              Import Data
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='anchor' />
              API
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='history' />
              History
            </Menu.Item>
            <Divider />
            <Menu.Item
                as='a'>
              <Icon name='plus' />
              Create New
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='clone' />
              Duplicate
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='edit' />
              Modify
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='undo alternate' />
              Rollback
            </Menu.Item>
            <Menu.Item
                as='a'>
              <Icon name='trash' />
              Delete
            </Menu.Item>
            <Divider />
            <Menu.Item
                as='a'>
              <Icon name='shield' />
              Access
            </Menu.Item>
          </Sidebar>

          <Sidebar.Pusher>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded style={{ height: '100%', overflow: 'hidden'}}>
                <Segment>
                  <Label as='a'>
                    <Icon name='mouse pointer' />
                    select
                    <Icon name='delete' />
                  </Label>
                  <Label as='a'>
                    <Icon name='filter' />
                    where
                    <Icon name='delete' />
                  </Label>
                  <Label as='a'>
                    <Icon name='sort' />
                    order by
                    <Icon name='delete' />
                  </Label>
                  <Label as='a' color='green'>
                    <Icon name='add' style={{marginRight: 0}}/>
                  </Label>
                </Segment>
                <GridLayout />
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    );
  }
}

export default Tables