
import React, { Component } from 'react'
import ReactDOM from 'react-dom'

import { AgGridReact } from 'ag-grid-react';
import 'ag-grid-community/dist/styles/ag-grid.css';
import 'ag-grid-community/dist/styles/ag-theme-balham.css';

class GridLayout extends Component {

  state = {
    columnDefs: [
        {headerName: "Make", field: "make"},
        {headerName: "Model", field: "model"},
        {headerName: "Price", field: "price"}

    ],
    rowData: [
        {make: "Toyota", model: "Celica", price: 35000},
        {make: "Ford", model: "Mondeo", price: 32000},
        {make: "Porsche", model: "Boxter", price: 72000}
    ]
  }

  render() {
    return (
      <div
        className="ag-theme-balham"
        style={{
          height: '100%',
          width: '100%',
        }}
      >
          <AgGridReact
              columnDefs={this.state.columnDefs}
              rowData={this.state.rowData}>
          </AgGridReact>
      </div>
    )
  }

  componentDidMount() {
    // For setting the context menu manually
    let dom = ReactDOM.findDOMNode(this)
    dom.querySelector('.ag-header').addEventListener('click', (x) => console.log('is clicked!'), false)
  }
}

export default GridLayout;