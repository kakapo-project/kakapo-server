
import ReactDataGrid from 'react-data-grid'

class DataGrid extends ReactDataGrid {
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