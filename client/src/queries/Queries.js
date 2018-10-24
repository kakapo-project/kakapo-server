
import React, { Component } from 'react'
import { Button, Card, Container, Form, Header, Grid, Icon, Image, Menu, Segment, Sidebar } from 'semantic-ui-react'

import CodeMirror from 'react-codemirror'


import 'codemirror/addon/hint/sql-hint'
import 'codemirror/lib/codemirror.css'
import 'codemirror/theme/darcula.css'


class Queries extends Component {

  render() {

    return (
      <Segment basic>
        <Form>
          <CodeMirror options={{
            theme: 'darcula',
            mode: 'text/x-mysql',
            lineNumbers: true,
            styleActiveLine: true,
          }} />
        </Form>
      </Segment>
    )

  }
}

export default Queries