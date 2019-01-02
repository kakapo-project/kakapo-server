
import React, { Component } from 'react'
import { Button, Card, Container, Dimmer, Divider, Dropdown, Form, Grid, Icon, Input, Loader, Image, Label, Menu, Segment, Select, Sidebar } from 'semantic-ui-react'

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

import QueryData from './QueryData'

const QueriesSidebar = (props) => (
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

const ParameterList = (props) => {
  const grouped = (arr, n) => {
    let result = []
    arr.map((x, idx) => {
      let g = idx / n >> 0
      result[g] = result[g] ? result[g].concat([x]) : [x]
    })
    return result
  }
  Array.prototype.grouped = function(n) { return grouped(this, n) }
  const N = 4

  return props.params.grouped(N).map((row, rowIdx) => (
    <Grid.Row key={rowIdx} style={{ paddingTop: '0.5rem', paddingBottom: '0.5rem' }}>
      {row.map((x, idx) => {
        let key = rowIdx * N + idx
        return (
          <Grid.Column key={key} width={16 / N >> 0}>
            <Input
              labelPosition='right'
              placeholder='Column Name'
              fluid
              value={x.value || ''}
              onChange={(e, data) => props.modifyParam(key, { value: data.value, type: 'string' })}
              action
            >
              <input />
              <Select
                compact
                defaultValue={DEFAULT_TYPE}
                options={ALL_TYPES.map(x => ({key: x, text: x, value: x}))}
                onChange={(e, data) => console.log('...')}
              />
              <Button icon='delete' color='orange' onClick={(e) => props.deleteParam(key)} />
            </Input>

          </Grid.Column>
        )
      })}
    </Grid.Row>
  ))
}
class Queries extends Component {

  state = {
    sidebarOpen: false,
    localStatement: null,
    statement: '',
    isRunningQuery: false,
    isTableLoaded: false,
    params: [],
    error: null,
    queryLoaded: false,
  }

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
      this.setState({ queryLoaded: true })
    }

    this.socket.onerror = (event) => {
      console.log('error')
      this.raiseError('Could not setup connection')
    }

    this.socket.onclose = (event) => {
      console.error('WebSocket closed: ', event)
      this.setState({ queryLoaded: false })
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
    this.props.loadedPage()
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
        <Header editor />
        <ErrorMsg error={this.state.error} onClose={(type) => this.closeErrorMessage(type)} types={this.errorMsgTypes}/>
        <Sidebar.Pushable className='basic attached' as={Segment} style={{height: 'calc(100vh - 5.15em)', border: 0}}>
          <QueriesSidebar
            visible={ this.props.isSidebarOpen }
          />

          <Sidebar.Pusher>
            <Dimmer active={false}>
              <Loader size='big'>Loading</Loader>
            </Dimmer>
            <Segment basic padded style={{}}>
              <Segment padded='very' style={{ minHeight: '100%' }}>
                { this.state.queryLoaded &&
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
                }
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
                      <Grid.Column>
                        <Button
                          icon
                          circular
                          color='black'
                          size='large'
                          floated='right'
                          onClick={() => this.setState({ params: this.state.params.concat([{ value: '', type: DEFAULT_TYPE }]) })}
                        >
                          <Icon  name='add' /> {/* TODO: parameters, technically, don't need the add */}
                        </Button>
                      </Grid.Column>
                    </Grid.Row>
                    <ParameterList
                      params={this.state.params}
                      modifyParam={(key, value) => this.setState({params:
                        [...this.state.params.slice(0, key), value, ...this.state.params.slice(key + 1) ]
                      })}
                      deleteParam={(key) => this.setState({params:
                        [...this.state.params.slice(0, key), ...this.state.params.slice(key + 1) ]
                      })}
                    />
                    <Divider hidden style={{ margin: '0.25rem' }}/>
                  </Grid>

                  {this.state.isTableLoaded ?
                    <>
                      <Divider />
                      <QueryData
                        columns={this.state.columns}
                        data={this.state.data}
                      />
                    </>
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

const mapStateToProps = (state) => ({
  isSidebarOpen: state.sidebar.isOpen,
  error: null,
})

const mapDispatchToProps = (dispatch) => ({
  loadedPage: () => dispatch(loadedPage('Queries')),
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Queries)