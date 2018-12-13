
import React, { Component } from 'react'
import { Button, Card, Container, Divider, Dropdown, Form, Grid, Icon, Input, Image, Label, Menu, Segment, Select, Sidebar } from 'semantic-ui-react'

import { connect } from 'react-redux'


import ErrorMsg from '../ErrorMsg'
import { WS_URL } from '../config'
import { DEFAULT_TYPE, ALL_TYPES } from '../actions/columns'


import { Controlled as CodeMirror } from 'react-codemirror2'
import _ from 'lodash'

import 'codemirror/addon/hint/sql-hint'
import 'codemirror/lib/codemirror.css'
import 'codemirror/theme/darcula.css'


import Header from '../Header.js'

import { loadedPage } from '../actions'


const ScriptsSidebar = (props) => (
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

class Scripts extends Component {

  state = {
    sidebarOpen: false,
    localStatement: null,
    statement: '',
    isRunningQuery: false,
    isTableLoaded: false,
    error: null,
  }

  setupConnection() {
    const { name } = this.props.match.params
    const url = `${WS_URL}/script/${name}`
    this.socket = new WebSocket(url);
    console.log('socket: ', this.socket)

    let sendGetQuery = {
      action: 'getScript',
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
        case 'getScript':
        case 'postScript':
          console.log('getTable: rawData: ', rawData)
          this.setState({
            statement: rawData.statement,
          })
          return
        case 'runScript': {
          console.log('runScript: rawData: ', rawData)
          return
        }
      }
    }
  }

  uploadText(value) {
    console.log('uploadText')
    const { name } = this.props.match.params

    let postScriptQuery = {
      action: 'postScript',
      data: {
        name: name,
        text: value
      }
    }
    this.socket.send(JSON.stringify(postScriptQuery))
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

  componentWillMount() {
    this.props.loadedPage()
  }

  componentDidMount() {
    this.setupConnection()
  }

  uploadEditorChange = _.debounce((value) => {
    this.uploadText(value)
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
        <Header editor />
        <ErrorMsg error={this.state.error} onClose={(type) => this.closeErrorMessage(type)} types={this.errorMsgTypes}/>
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)', border: 0}}>
          <ScriptsSidebar sidebarOpen={this.props.isSidebarOpen()} />

          <Sidebar.Pusher>
            <Segment basic padded style={{}}>
              <Segment padded='very' style={{ minHeight: '100%' }}>
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
                    <Grid.Row>
                      <Grid.Column>
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
                    </Grid.Row>
                  </Grid>

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

const mapStateToProps = (state) => ({
  isSidebarOpen: () => state.sidebar.isOpen,
  error: null,
})

const mapDispatchToProps = (dispatch) => ({
  loadedPage: () => dispatch(loadedPage('Scripts')),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Scripts)