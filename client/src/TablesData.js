

import React, { Component } from 'react'
import {
  Button,
  Card,
  Container,
  Grid,
  Header,
  Icon,
  Image,
  Input,
  Label,
  Menu,
  Pagination,
  Segment,
  Sidebar,
  Table } from 'semantic-ui-react'


//TODO: put this in another route, this component shouldn't have a sidebar
class TableData extends Component {

  render() {

    let nColumns = 3;

    return (
      <Table celled>
        <Table.Header>
          <Table.Row>
            <Table.HeaderCell>Header</Table.HeaderCell>
            <Table.HeaderCell>Header</Table.HeaderCell>
            <Table.HeaderCell>Header</Table.HeaderCell>
          </Table.Row>
        </Table.Header>

        <Table.Body>
          <style>{`
            .ui.celled.table tr > td {
              padding: 3px;
            }
            .ui.input > input {
              border: 0px;
            }
            .ui.input > input {
              height: 100%;
              width: 100%;
            }

            .ui.table td.warning .ui.input,
            .ui.table tr.warning .ui.input,
            .ui.table td.warning .ui.input > input,
            .ui.table tr.warning .ui.input > input {
              background: #fffaf3!important;
              color: #573a08!important;
            }

            .ui.table td.error .ui.input,
            .ui.table tr.error .ui.input,
            .ui.table td.error .ui.input > input,
            .ui.table tr.error .ui.input > input {
              background: #fff6f6!important;
              color: #9f3a38!important;
            }

            .ui.table td.positive .ui.input,
            .ui.table tr.positive .ui.input,
            .ui.table td.positive .ui.input > input,
            .ui.table tr.positive .ui.input > input {
              background: #fcfff5!important;
              color: #2c662d!important;
            }

            .ui.table td.negative .ui.input,
            .ui.table tr.negative .ui.input,
            .ui.table td.negative .ui.input > input,
            .ui.table tr.negative .ui.input > input {
              background: #fff6f6!important;
              color: #9f3a38!important;
            }

          `}</style>
          { [...Array(25).keys()].map(x =>
            <Table.Row negative>
              <Table.Cell>
                <Input placeholder='Search...' />
              </Table.Cell>
              <Table.Cell>
                <Input placeholder='Search...' />
              </Table.Cell>
              <Table.Cell>
                <Input placeholder='Search...' />
              </Table.Cell>
            </Table.Row>
          )}
        </Table.Body>

        <Table.Footer>
          <Table.Row>
            <Table.HeaderCell colSpan={nColumns}>
              <Pagination
                floated='right'
                defaultActivePage={5}
                siblingRange={1}
                boundaryRange={1}
                ellipsisItem={{ content: <Icon name='ellipsis horizontal' />, icon: true }}
                firstItem={{ content: <Icon name='angle double left' />, icon: true }}
                lastItem={{ content: <Icon name='angle double right' />, icon: true }}
                prevItem={{ content: <Icon name='angle left' />, icon: true }}
                nextItem={{ content: <Icon name='angle right' />, icon: true }}
                totalPages={10}
              />
            </Table.HeaderCell>
          </Table.Row>
        </Table.Footer>
      </Table>
    )
  }
}

export default TableData