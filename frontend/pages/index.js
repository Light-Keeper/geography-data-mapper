import { MapWithNoSSR } from '../components/MapWithNoSSR'
import { Callout, Intent, Slider } from '@blueprintjs/core'
import css from './containers.module.scss'
import SelectDatasource from '../components/common/SelectDatasource'
import { useState } from 'react'

function HomePage() {
  const [selectedDatasource, setSelectedDatasource] = useState(undefined)
  const [limit, setLimit] = useState(100)
  const order = selectedDatasource?.metadata?.recommendedOrder

  return (
    <div className={css.fullpage}>
      <div className={css.sidebar}>
        <Callout title='Welcome to the Map' icon='info-sign' intent={Intent.PRIMARY}>
          <p>Experience the joy of discovering some random points across the Globe.</p>
        </Callout>
        <div className={css.buttons}>
          <SelectDatasource selectedDatasource={selectedDatasource} setSelectedDatasource={setSelectedDatasource} />
          <div style={{ padding: '10px', marginTop: '10px' }}>
            <span>
              <b>Number of points to show:</b> {limit}
            </span>
            <Slider
              min={10}
              max={1000}
              stepSize={10}
              labelStepSize={100}
              labelRenderer={false}
              onChange={setLimit}
              value={limit}
            />
          </div>
        </div>
      </div>
      <div className={css.main}>
        <MapWithNoSSR selectedDatasource={selectedDatasource} limit={limit} order={order} />
      </div>
    </div>
  )
}

export default HomePage
