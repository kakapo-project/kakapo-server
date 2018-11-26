import React, { Component } from 'react'
import {
  Button,
  Card,
  Container,
  Divider,
  Dimmer,
  Loader,
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
import ErrorMsg from '../ErrorMsg'


import { WS_URL } from '../config'

const TableSidebase = (props) => (
  <Sidebar
    as={Menu}
    animation='overlay'
    icon='labeled'
    inverted
    direction='right'
    vertical
    visible={props.visible}
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
)
class Tables extends Component {

  //TODO: Filters....

  state = {
    sidebarOpen: false,
    data: null,
    columns: null,
    error: null,
  }


  toggleSidebar() {
    this.setState({
      ...this.state,
      sidebarOpen: !this.state.sidebarOpen,
    })
  }

  raiseError(msg) {
    this.setState({ error: msg })
  }

  errorMsgTypes = ['Retry', 'Go Back']
  closeErrorMessage(type) {
    switch (type) {
      case this.errorMsgTypes[0]:
        this.setupConnection()
        this.setState({ error: null })
        return
      case this.errorMsgTypes[1]:
        this.props.history.push('/')
        return
    }
  }

  setupConnection() {
    const { name } = this.props.match.params
    const url = `${WS_URL}/table/${name}`
    this.socket = new WebSocket(url);
    console.log('socket: ', this.socket)

    let sendGetTable = {
      action: 'getTable',
    }
    let sendGetTableData = {
      action: 'getTableData',
      begin: 0,
      end: 500,
    }

    this.socket.onopen = (event) => {
      this.socket.send(JSON.stringify(sendGetTable))
    }

    this.socket.onerror = (event) => {
      this.raiseError('Could not setup connection')
    }

    this.socket.onclose = (event) => {
      console.error('WebSocket closed: ', event)
    }

    this.socket.onmessage = (event) => {
      let incomingData = JSON.parse(event.data)
      let oldData = this.state.data || []
      let oldDataKeys = this.state.keys || new Set()

      let action = incomingData.action
      let rawData = incomingData.data

      switch (action) {
        case 'getTable': {
          let schema = rawData.schema
          let columns = schema.columns
          let constraint = schema.constraint

          let key = constraint.map(x => x.key).map(x => x)
          if (key.length !== 1) {
            this.raiseError('This table does not have any keys')
            return
          }
          console.log('table: ', rawData)
          this.setState({
            tableInfoColumns: columns,
            tableInfoKey: key[0],
          })
          this.socket.send(JSON.stringify(sendGetTableData))
          return
        }
        case 'getTableData':
        case 'update':
        case 'create':
          let data = rawData.data
          let columns = rawData.columns
          console.log('columns: ', columns)
          console.log('data: ', data)
          let keyIndex = columns.findIndex(x => x === this.state.tableInfoKey)
          if (keyIndex.length === -1) {
            this.raiseError('Unknown error: Database did not return the proper columns')
            return
          }

          const findIndex = (key) => oldData.findIndex(x => key === x[keyIndex])

          data.map((x) => {
            let key = x[keyIndex]
            console.log('key: ', key)
            if (oldDataKeys.has(key)) {
              //update
              let index = findIndex(key) //O(n)
              oldData[index] = x
            } else {
              //insert
              oldData.push(x)
            }

            oldDataKeys.add(key)
          })

          let indices = oldData.map((_, idx) => idx + 1)

          this.setState({
            data: oldData,
            indices: indices,
            keys: oldDataKeys,
            columns: columns,
            keyIndex: keyIndex,
          })
          return
      }
    }
  }

  getNewRows() {
    return this.state.newRows || []
  }

  addRow(afterIdx) {
    let { indices, data } = this.state
    indices.splice(afterIdx + 1, 0, <Icon name='minus' /> )
    data.splice(afterIdx + 1, 0, data[0].map(x => ''))

    let newRows = this.getNewRows()
    newRows = newRows.map(x => {
      if (x > afterIdx) {
        return x + 1
      } else {
        return x
      }
    })
    newRows.push(afterIdx + 1)
    console.log('newRows: ', newRows)
    this.setState({
      data: data,
      indices: indices,
      newRows: newRows,
    })
  }

  parseColumn(data, colName) {
    let table = this.state.tableInfoColumns
    let column = table.filter(x => x.name === colName)[0] //TODO: error checking
    if (column.dataType === 'integer') {
      if (!data) {
        return null
      } else {
        return parseInt(data)
      }
    } else {
      return data
    }
  }

  updateValue(input, rowKey, colKey) {
    //TODO: delete row if the key is changed
    let newRows = this.getNewRows()
    let data = {}
    let key = this.state.tableInfoKey
    let keyIndex = this.state.keyIndex

    console.log('newRows: ', newRows)
    console.log('rowKey: ', rowKey)
    if (newRows.includes(rowKey)) {
      if (colKey === keyIndex) {
        let newData = this.state.data
        let newKeys = this.state.keys

        this.state.columns.map((x, idx) => {
          if (colKey === idx) {
            data[x] = this.parseColumn(input, x)
            newKeys.add(data[x])
          } else {
            data[x] = this.parseColumn(this.state.data[rowKey][idx], x)
          }
        })


        newData[rowKey][colKey] = input
        this.setState({ data: newData, keys: newKeys })

        let sendData = {
          action: 'create',
          data: data
        }
        console.log('sending data: ', sendData)
        this.socket.send(JSON.stringify(sendData))
      } else {
        let newData = this.state.data
        newData[rowKey][colKey] = input
        this.setState({ data: newData })
      }

    } else {

      data[key] = this.state.data[rowKey][keyIndex]
      this.state.columns.map((x, idx) => {
        if (colKey === idx) {
          data[x] = this.parseColumn(input, x)
        }
      })

      let sendData = {
        action: 'update',
        data: data
      }
      console.log('sending data: ', sendData)
      this.socket.send(JSON.stringify(sendData))
    }
  }

  componentDidMount() {
    this.setupConnection()
  }

  render() {
    return (
      <div>
        <Header
          editor
          sidebarOpen={this.state.sidebarOpen}
          onToggle={() => this.toggleSidebar()}
        />
        <ErrorMsg error={this.state.error} onClose={(type) => this.closeErrorMessage(type)} types={this.errorMsgTypes}/>
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)'}}>
          <TableSidebase visible={this.state.sidebarOpen} />

          <Sidebar.Pusher>
            <Dimmer active={this.state.data === null}>
              <Loader size='big'>Loading</Loader>
            </Dimmer>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded style={{ height: '100%', overflowY: 'scroll', overflowX: 'hidden'}}>
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
                <GridLayout
                  data={this.state.data}
                  columns={this.state.columns}
                  indices={this.state.indices}
                  addRow={(afterIdx) => this.addRow(afterIdx)}
                  updateValue={(input, rowKey, colKey) => this.updateValue(input, rowKey, colKey)}
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