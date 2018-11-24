

export function getData() {
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

  let data = [...Array(100).keys()].map(x => (
    {
      'id': x,
      'name': 'I.P. Freely',
      'age': 69,
      'data': '{}',
      'last_visited': new Date(),
      'joined': new Date(),
      'is_admin': true,
    }
  ))



  return { schema, data }
}

export function getColumnsWithKey() {
  let { schema } = getData()
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


export function getColumns() {
  let { columns, key, foreignKeys } = getColumnsWithKey()

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


export function getIndices() {

  let { key } = getColumnsWithKey()

  let { data } = getData()

  let indices = null
  if (key !== null) {
    indices = data.map(x => x[key])
  }

  return indices
}

export function getRows() {
  let { columns } = getColumnsWithKey()

  let { data } = getData()

  const orderBasedOnColumn = (row) => columns.map(column => row[column.name])
  return data.map(x => orderBasedOnColumn(x))
}