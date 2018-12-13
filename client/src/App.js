
import React, { Component } from 'react'
import { Switch, Route } from 'react-router-dom'

import Home from './Home'
import Tables from './table/Tables'
import Queries from './queries/Queries'
import Scripts from './scripts/Scripts'
import Login from './Login'

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
          <Route path='/login' component={Login}/>
          <Route path='/tables/:name' component={Tables}/>
          <Route path='/queries/:name' component={Queries}/>
          <Route path='/scripts/:name' component={Scripts}/>
        </Switch>
      </main>
    )
  }
}

export default App