import { MapWithNoSSR } from '../components/MapWithNoSSR'
import { Callout, Intent } from "@blueprintjs/core";
import css from './containers.module.scss'
import SelectDatasource from "../components/common/SelectDatasource";
import { useState } from "react";

function HomePage() {
  const [selectedDatasource, setSelectedDatasource] = useState(undefined);

  return (
    <div className={css.fullpage}>
      <div className={css.sidebar}>
        <Callout title="Welcome to the Map" icon="info-sign" intent={Intent.PRIMARY}>
          <p>
            Experience the joy of discovering some random points across Ukraine.
          </p>
        </Callout>
        <div className={css.buttons}>
          <SelectDatasource selectedDatasource={selectedDatasource} setSelectedDatasource={setSelectedDatasource}/>
        </div>
      </div>
      <div className={css.main}>
        <MapWithNoSSR selectedDatasource={selectedDatasource}/>
      </div>
    </div>
  )
}

export default HomePage
