
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

  getHandsontableType(type) {

    switch (type) {
      case 'Boolean':
        return 'checkbox'
      default:
        return 'text'
    }
  }


  renderColumnName(column) {

    const renderKeySymbol = (column) => {
      if (column.isPrimaryKey) {
        return '  <i aria-hidden="true" class="key icon">'
      } else if (column.isForeignKey) {
        return '  <i aria-hidden="true" class="linkify icon">'
      } else {
        return ''
      }
    }

    return `<strong>${column.name}</strong>${renderKeySymbol(column)}`
  }

  renderIndexForRowWithNoKey() {
    return '<i aria-hidden="true" class="question icon">'
  }

  renderColumn(column) {
    return {
      headerName: this.renderColumnName(column),
      field: column.name,
    }
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
        },
        {
          reference: {
            column: 'age',
            foreignTable: 'other_table',
            foreignColumn: 'other_table_id',
          },
        },
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
        'is_admin': true,
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
        'is_admin': true,
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
        'is_admin': true,
      },
    ]

    return { schema, data }
  }

  getColumnMetadata(column) {
    return {
      data: column.name,
      type: this.getHandsontableType(column.dataType),
    }
  }

  getColumnsWithKey() {
    let { schema } = this.getData()
    let { columns, constraints } = schema

    let keyConstraints = constraints.map(x => x.key).filter(x => x !== undefined)
    let key = null
    if (keyConstraints.length !== 0) {
      key = keyConstraints[0]
    } else if (keyConstraints.length > 1) {
      console.log('warning, more than one primary key found. Server is wrong')
    }

    let foreignKeyConstraints = constraints.map(x => x.reference).filter(x => x !== undefined)
    let foreignKeys = foreignKeyConstraints.map(x => x.column)

    return { columns, key, foreignKeys }
  }


  getColumns() {
    let { columns, key, foreignKeys } = this.getColumnsWithKey()

    let columnsByName = {}
    for (let column of columns) {
      columnsByName[column.name] = {...column, isPrimaryKey: false, isForeignKey: false}
    }

    // add in the primary key
    if (key in columnsByName) {
      columnsByName[key] = {...columnsByName[key], isPrimaryKey: true}
    } else {
      console.log('warning, could not find key in any of the columns. Server is wrong')
    }

    // add in the foreign keys
    for (let key of foreignKeys) {
      if (key in columnsByName) {
        columnsByName[key] = {...columnsByName[key], isForeignKey: true}
      } else {
        console.log('warning, could not find key in any of the columns. Server is wrong')
      }
    }

    return Object.values(columnsByName)
  }


  getIndices() {

    let { key } = this.getColumnsWithKey()

    let { data } = this.getData()

    let indices = null
    if (key !== null) {
      indices = data.map(x => x[key])
    }

    return indices
  }

  getRows() {
    let { columns } = this.getColumnsWithKey()

    let { data } = this.getData()

    //const orderBasedOnColumn = (row) => columns.map(column => row[column.name])

    return data //data.map(row => orderBasedOnColumn(row))
  }

  render() {

    console.log('this.state.columnDefs: ', this.state.columnDefs)
    console.log('this.getColumns(): ', this.getColumns())
    return (
      <div
        className="ag-theme-balham"
        style={{
          height: '100%',
          width: '100%',
        }}
      >
          <AgGridReact
              columnDefs={this.getColumns().map(x => this.renderColumn(x))}
              rowData={this.getRows()}>
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