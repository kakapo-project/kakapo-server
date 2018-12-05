
import React, { Component, useState } from 'react'
import {
  Button,
  Card,
  Container,
  Divider,
  Dimmer,
  Loader,
  Grid,
  Icon,
  Image,
  Input,
  Label,
  Menu,
  Pagination,
  Segment,
  Sidebar,
  Table
} from 'semantic-ui-react'


import GridLayout from './GridLayout.js'

import Header from '../Header.js'
import ErrorMsg from '../ErrorMsg'


import { WS_URL } from '../config'
import { connect } from 'react-redux'

import { requestingTableData } from '../actions'

import ReactDataGrid from 'react-data-grid'

import faker from 'faker'

function createFakeRow(index) {
  return {
    id: index,
    avartar: faker.image.avatar(),
    county: faker.address.county(),
    email: faker.internet.email(),
    title: faker.name.prefix(),
    firstName: faker.name.firstName(),
    lastName: faker.name.lastName(),
    street: faker.address.streetName(),
    zipCode: faker.address.zipCode(),
    date: faker.date.past().toLocaleDateString(),
    jobTitle: faker.name.jobTitle(),
    catchPhrase: faker.company.catchPhrase(),
    companyName: faker.company.companyName(),
    jobArea: faker.name.jobArea(),
    jobType: faker.name.jobType()
  };
}

function createRowData(count) {
  return [...Array(count).keys()].map(i => createFakeRow(i));
}

const { ContextMenu, MenuItem, SubMenu, ContextMenuTrigger } = Menu;

const defaultColumnProperties = {
  sortable: true,
  width: 120
};

const columns = [
  {
    key: "id",
    name: "ID",
    sortDescendingFirst: true
  },
  {
    key: "title",
    name: "Title"
  },
  {
    key: "firstName",
    name: "First Name"
  },
  {
    key: "lastName",
    name: "Last Name"
  },
  {
    key: "email",
    name: "Email"
  },
  {
    key: "street",
    name: "Street"
  },
  {
    key: "zipCode",
    name: "ZipCode"
  },
  {
    key: "date",
    name: "Date"
  },
  {
    key: "jobTitle",
    name: "Job Title"
  },
  {
    key: "catchPhrase",
    name: "Catch Phrase"
  },
  {
    key: "jobArea",
    name: "Job Area"
  },
  {
    key: "jobType",
    name: "Job Type"
  }
].map(c => ({ ...c, ...defaultColumnProperties }));

const ROW_COUNT = 50;

function ExampleContextMenu({
  idx,
  id,
  rowIdx,
  onRowDelete,
  onRowInsertAbove,
  onRowInsertBelow
}) {
  return (
    <ContextMenu id={id}>
      <MenuItem data={{ rowIdx, idx }} onClick={onRowDelete}>
        Delete Row
      </MenuItem>
      <SubMenu title="Insert Row">
        <MenuItem data={{ rowIdx, idx }} onClick={onRowInsertAbove}>
          Above
        </MenuItem>
        <MenuItem data={{ rowIdx, idx }} onClick={onRowInsertBelow}>
          Below
        </MenuItem>
      </SubMenu>
    </ContextMenu>
  );
}

const deleteRow = rowIdx => rows => {
  const nextRows = [...rows];
  nextRows.splice(rowIdx, 1);
  return nextRows;
};

const insertRow = rowIdx => rows => {
  const newRow = createFakeRow("-");
  const nextRows = [...rows];
  nextRows.splice(rowIdx, 0, newRow);
  return nextRows;
};



//componentDidMount() {
//  this.props.requestingTableData()
//}


const Example = (props) => {
  const result = props.initialRows
  const rows = result[0];
  const setRows = result[1];
  return (
    <ReactDataGrid
      columns={columns}
      rowGetter={i => rows[i]}
      rowsCount={ROW_COUNT}
      minHeight={500}
      RowsContainer={ContextMenuTrigger}
    />
  );
}


export default (props) => <Example initialRows={createRowData(50)} />