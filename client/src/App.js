
import React, { Component } from 'react'
import { Switch, Route } from 'react-router-dom'

import Home from './Home.js'
import Tables from './table/Tables.js'
import Queries from './queries/Queries.js'
import Login from './Login.js'

class App extends Component {
  render() {
    return (
      <main>
        <style>
          {`
            i.scheme-green.icon {
              color: #005322!important;
            }
            i.inverted.scheme-green.icon {
              color: #005322!important;
            }
            i.inverted.bordered.scheme-green.icon, i.inverted.circular.scheme-green.icon {
              background-color: #005322!important;
              color: #fff!important;
            }
          `}
        </style>
        <Switch>
          <Route exact path='/' component={Home}/>
          <Route path='/tables/:name' component={Tables}/>
          <Route path='/queries' component={Queries}/>
          <Route path='/login' component={Login}/>
        </Switch>
      </main>
    )
  }
}

export default App