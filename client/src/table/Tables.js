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

import 'handsontable/dist/handsontable.full.css'

import { HotTable } from '@handsontable/react'

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

  renderColumn(column) {
    return `${column.name}  <small>${this.renderTypeSymbol(column.dataType)}</small>`
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
        }
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
        'is_admin': false,
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
        'is_admin': false,
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
        'is_admin': false,
      },
    ]

    return { schema, data }
  }

  getColumnsWithKey() {
    let { schema } = this.getData()
    let { columns, constraints } = schema

    let keyConstraints = constraints.map(x => x.key).filter(x => x !== undefined)
    let key = null
    if (keyConstraints.length !== 0) {
      key = keyConstraints[0]
    }

    return { columns, key }
  }


  getColumns() {
    let { columns } = this.getColumnsWithKey()

    return columns
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

    const orderBasedOnColumn = (row) => columns.map(column => row[column.name])

    return data.map(row => orderBasedOnColumn(row))
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
                <HotTable
                  data={this.getRows()}
                  colHeaders={this.getColumns().map(x => this.renderColumn(x))}
                  rowHeaders={this.getIndices()}
                  /*stretchH="all"*/
                  autoWrapRow={true}
                  style={{width: '100%', height: '100%'}}
                />
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    );
  }
}

export default Tables