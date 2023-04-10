import { MapWithNoSSR } from '../components/MapWithNoSSR'
import { Callout, Colors, Intent, Slider } from "@blueprintjs/core";
import css from './containers.module.scss'
import SelectDatasource from "../components/common/SelectDatasource";
import { useCallback, useEffect, useState } from "react";

const ColoringComponent = () => {
  return null;
}

function parseColor(input) {
  if (input.substr(0,1)==="#") {
    var collen=(input.length-1)/3;
    var fact=[17,1,0.062272][collen-1];
    return [
      Math.round(parseInt(input.substr(1,collen),16)*fact),
      Math.round(parseInt(input.substr(1+collen,collen),16)*fact),
      Math.round(parseInt(input.substr(1+2*collen,collen),16)*fact)
    ];
  }
  else return input.split("(")[1].split(")")[0].split(",").map(x=>+x);
}

function HomePage() {
  const [selectedDatasource, setSelectedDatasource] = useState(undefined);
  const [limit, setLimit] = useState(100);
  const [order, setOrder] = useState(undefined);

  useEffect(() => {
    setOrder(selectedDatasource?.metadata?.recommendedOrder)
  }, [selectedDatasource]);

  return (
    <div className={css.fullpage}>
      <div className={css.sidebar}>
        <Callout title="Welcome to the Map" icon="info-sign" intent={Intent.PRIMARY}>
          <p>
            Experience the joy of discovering some random points across the Globe.
          </p>
        </Callout>
        <div className={css.buttons}>
          <SelectDatasource selectedDatasource={selectedDatasource} setSelectedDatasource={setSelectedDatasource}/>
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
          <ColoringComponent/>
        </div>
      </div>
      <div className={css.main}>
        <MapWithNoSSR selectedDatasource={selectedDatasource} limit={limit} order={order}/>
      </div>
    </div>
  )
}

export default HomePage
