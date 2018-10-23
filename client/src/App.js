
import React, { Component } from 'react'
import { Switch, Route } from 'react-router-dom'

import Home from './Home.js'
import Tables from './table/Tables.js'
import Queries from './queries/Queries.js'

class App extends Component {
  render() {
    return (
      <main>
        <Switch>
          <Route exact path='/' component={Home}/>
          <Route path='/tables' component={Tables}/>
          <Route path='/queries' component={Queries}/>
        </Switch>
      </main>
    )
  }
}

export default App