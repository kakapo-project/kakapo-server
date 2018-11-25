import React, { Component } from 'react'
import { Icon, Label, Menu, Table } from 'semantic-ui-react'


const RenderedRow = (props) => {

}

class DataGrid extends Component {
  render() {
    let { columns, rows, getData} = this.props
    return (
      <Table celled fixed style={{userSelect: 'none'}}>
        <style>
          {`
            .ui.table thead th:first-child {
              background: #1b1c1d;
              color: rgba(255,255,255,.9);
              border-color: none;
            }
            .ui.table thead th {
              background: #333;
              color: rgba(255,255,255,.9);
              border: none;
              border-color: rgba(255,255,255,.1)!important;
              border-left: none !important;
              border-right: 1px solid rgba(34,36,38,.1);
            }
            .ui.table tbody tr td:first-child {
              background: #333;
              color: rgba(255,255,255,.9);
              border: none;
              border-color: rgba(255,255,255,.1)!important;
              border-top: none !important;
              border-bottom: 1px solid rgba(34,36,38,.1);
            }
          `}
        </style>
        <Table.Header >
          <Table.Row>
            <Table.HeaderCell />
            {columns}
          </Table.Row>
        </Table.Header>

        <Table.Body>
          {
            rows.map((rowKey, rowIdx) =>
              <Table.Row key={rowIdx}>
                {rowKey}
                {columns.map((_, colIdx) => getData(rowIdx, colIdx)) }
              </Table.Row>
            )
          }
        </Table.Body>
      </Table>
    )
  }
}

export default DataGrid