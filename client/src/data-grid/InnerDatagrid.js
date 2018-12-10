
import ReactDataGrid from 'react-data-grid'

class DataGrid extends ReactDataGrid {

  selectStart = (cellPosition) => {
    const cellIsBeingEdited = document.getElementsByClassName('rdg-editor-container').length === 1
    const SELECT_START = 'SELECT_START';
    console.log('cellIsBeingEdited: ', cellIsBeingEdited)
    if (!cellIsBeingEdited) {
      this.eventBus.dispatch(SELECT_START, cellPosition);
    }
  }

  componentDidMount() {
    this._mounted = true;
    this.dataGridComponent = document.getElementsByClassName('react-grid-Container')[0] //assumes only one react datagrid component exists
    window.addEventListener('resize', this.metricsUpdated);
    if (this.props.cellRangeSelection) {
      this.dataGridComponent.addEventListener('mouseup', this.onWindowMouseUp);
    }
    this.metricsUpdated();
  }

  componentWillUnmount() {
    this._mounted = false;
    window.removeEventListener('resize', this.metricsUpdated);
    this.dataGridComponent.removeEventListener('mouseup', this.onWindowMouseUp);
  }
}

export default DataGrid