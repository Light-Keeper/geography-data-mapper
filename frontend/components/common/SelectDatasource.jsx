import { Select2 } from '@blueprintjs/select'
import { Button, MenuItem } from '@blueprintjs/core'
import { useDatasources } from '../../models/datasources'
import { useEffect } from "react";

const SelectDatasource = ({ selectedDatasource, setSelectedDatasource }) => {
  let { data, isLoading } = useDatasources()
  let text = selectedDatasource?.name || 'Select Datasource'

  let someSelection = Boolean(selectedDatasource)

  useEffect(() => {
    if (!someSelection && data?.length) {
      setSelectedDatasource(data[0])
    }
  }, [data, someSelection, setSelectedDatasource]);

  let disabled = isLoading || !data?.length

  return (
    <Select2
      popoverProps={{ minimal: true, usePortal: false }}
      items={data || []}
      itemRenderer={(item, { handleClick, handleFocus }) => <MenuItem
        key={item.id}
        text={item.name}
        onClick={handleClick}
        onFocus={handleFocus}
        active={item.id === selectedDatasource?.id}
        roleStructure="listoption"
        icon={item.id === selectedDatasource?.id ? 'tick' : 'blank'}
      />}
      onItemSelect={setSelectedDatasource}
      filterable={false}
    >
      <Button text={text} rightIcon='caret-down' loading={isLoading} disabled={disabled} />
    </Select2>
  )
}

export default SelectDatasource
