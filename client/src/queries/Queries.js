
import React, { Component } from 'react'
import { Button, Card, Container, Divider, Form, Grid, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

import ErrorMsg from '../ErrorMsg'
import { WS_URL } from '../config'

import { Controlled as CodeMirror } from 'react-codemirror2'
import _ from 'lodash'

import 'codemirror/addon/hint/sql-hint'
import 'codemirror/lib/codemirror.css'
import 'codemirror/theme/darcula.css'

import GridLayout from '../table/GridLayout.js'

import Header from '../Header.js'

const QueriesSidebar = (props) => (
  <Sidebar
    as={Menu}
    animation='overlay'
    icon='labeled'
    inverted
    direction='right'
    vertical
    visible={props.sidebarOpen}
    width='thin'
  >
    <Menu.Item
        as='a'>
      <Icon name='download' />
      Download File
    </Menu.Item>
    <Menu.Item
        as='a'>
      <Icon name='upload' />
      Upload File
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
class Queries extends Component {

  state = {
    sidebarOpen: false,
    localStatement: null,
    statement: '',
    isRunningQuery: false,
    isTableLoaded: false,
    error: null,
  }

  getNa

  setupConnection() {
    const { name } = this.props.match.params
    const url = `${WS_URL}/query/${name}`
    this.socket = new WebSocket(url);
    console.log('socket: ', this.socket)

    let sendGetQuery = {
      action: 'getQuery',
    }

    this.socket.onopen = (event) => {
      this.socket.send(JSON.stringify(sendGetQuery))
    }

    this.socket.onerror = (event) => {
      console.log('error')
      this.raiseError('Could not setup connection')
    }

    this.socket.onclose = (event) => {
      console.error('WebSocket closed: ', event)
    }

    this.socket.onmessage = (event) => {
      let incomingData = JSON.parse(event.data)

      let action = incomingData.action
      let rawData = incomingData.data

      switch (action) {
        case 'getQuery':
        case 'postQuery':
          console.log('getTable: rawData: ', rawData)
          this.setState({
            statement: rawData.statement,
          })
          return
        case 'runQuery': {
          console.log('runQuery: rawData: ', rawData)

          let data = rawData.data
          let columns = rawData.columns
          let keys = data.map((x, idx) => idx)

          this.setState({
            data: data,
            columns: columns,
            keys: keys,
            isRunningQuery: false,
            isTableLoaded: true,
          })
          return
        }
      }
    }
  }

  uploadStatement(value) {
    console.log('uploadStatement')
    const { name } = this.props.match.params

    let sendPostQuery = {
      action: 'postQuery',
      data: {
        name: name,
        statement: value
      }
    }
    this.socket.send(JSON.stringify(sendPostQuery))
  }

  updateEvent = _.debounce((e) => console.log('e: ', e), 500)

  toggleSidebar() {
    this.setState({
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

  runQuery() {
    this.setState({ isRunningQuery: true })

    let sendRunQuery = {
      action: 'runQuery',
      params: []
    }
    this.socket.send(JSON.stringify(sendRunQuery))
  }

  componentDidMount() {
    this.setupConnection()
  }

  uploadEditorChange = _.debounce((value) => {
    this.uploadStatement(value)
  }, 500)

  render() {
    return (
      <div>
        <style>
          {`
            .react-codemirror2 > div.CodeMirror {
              border-radius: 5px;
            }
          `}
        </style>
        <Header
          editor
          sidebarOpen={this.state.sidebarOpen}
          onToggle={() => this.toggleSidebar()}
        />
        <ErrorMsg error={this.state.error} onClose={(type) => this.closeErrorMessage(type)} types={this.errorMsgTypes}/>
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)'}}>
          <QueriesSidebar sidebarOpen={this.state.sidebarOpen} />

          <Sidebar.Pusher>
            <Segment basic padded style={{ height: 'calc(100vh - 8em)' }}>
              <Segment padded='very' style={{ height: '100%' }}>
                <Form>
                  <CodeMirror
                    options={{
                      theme: 'darcula',
                      mode: 'text/x-mysql',
                      lineNumbers: true,
                      styleActiveLine: true,
                    }}
                    autoSave
                    value={this.state.localStatement || this.state.statement}
                    onBeforeChange={(editor, data, value) => {
                      this.setState({ localStatement: value })
                      this.uploadEditorChange(value)
                    }}
                    onChange={(editor, data, value) => {
                      console.log('statment        changed: ', editor, data, value) //TODO: when to use this
                    }}
                  />
                </Form>
                <Segment>
                  <Grid columns='equal'>
                    <Grid.Column width={4}>
                      <Button
                        color='black'
                        icon
                        size='large'
                        floated='left'
                        labelPosition='right'
                        loading={this.state.isRunningQuery}
                        onClick={(e) => this.runQuery()}
                      >
                        Run
                        {this.state.isRunningQuery ? <></> :
                          <Icon color='green' name='play' />
                        }
                      </Button>
                    </Grid.Column>
                    <Grid.Column>
                    </Grid.Column>
                    <Grid.Column width={4}>
                      <Button icon circular color='black' size='large' floated='right'>
                        <Icon  name='add' /> {/* TODO: parameters, technically, don't need the add */}
                      </Button>
                    </Grid.Column>
                  </Grid>
                  {this.state.isTableLoaded ?
                     <GridLayout
                      data={this.state.data}
                      columns={this.state.columns}
                      indices={this.state.keys}
                      addRow={(afterIdx) => console.log('not implemented')}
                      updateValue={(input, rowKey, colKey) => console.log('not implemented')}
                    />
                    : <></>
                  }

                </Segment>
              </Segment>
            </Segment>
          </Sidebar.Pusher>
        </Sidebar.Pushable>
      </div>
    )
  }

  // Generate Query fields
  // Run Button
  // Dry Run Button
  // NOTE: Auto run if no changes within 5 seconds
}

export default Queries