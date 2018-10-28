import React, { Component } from 'react'
import { Icon, Label, Menu, Table } from 'semantic-ui-react'


class DataGrid extends Component {
  render() {
    let { columns, rows, getData} = this.props
    return (
      <Table celled style={{userSelect: 'none'}}>
        <Table.Header>
          <Table.Row>
            <Table.HeaderCell />
            {columns}
          </Table.Row>
        </Table.Header>

        <Table.Body>
          {
            rows.map(x =>
              <Table.Row key={x.key}>
                {x}
                {columns.map(col => getData(x.key, col.key))}
              </Table.Row>
            )
          }
          {/*
          <Table.Row>
            <Table.Cell>First</Table.Cell>
            <Table.Cell>Cell</Table.Cell>
            <Table.Cell>Cell</Table.Cell>
          </Table.Row>
          <Table.Row>
            <Table.Cell>Cell</Table.Cell>
            <Table.Cell>Cell</Table.Cell>
            <Table.Cell>Cell</Table.Cell>
          </Table.Row>
          <Table.Row>
            <Table.Cell>Cell</Table.Cell>
            <Table.Cell>Cell</Table.Cell>
            <Table.Cell>Cell</Table.Cell>
          </Table.Row>
          */}
        </Table.Body>
      </Table>
    )
  }
}

export default DataGrid