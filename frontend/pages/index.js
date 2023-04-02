import { MapWithNoSSR } from '../components/MapWithNoSSR'
import { Button, Callout, Intent } from "@blueprintjs/core";
import css from './containers.module.scss'

function HomePage() {
  return (
    <div className={css.fullpage}>
      <div className={css.sidebar}>
        <Callout title="Welcome to the Map" icon="info-sign" intent={Intent.PRIMARY}>
          <p>
            Experience the joy of discovering 100 random locations across Ukraine.
          </p>
        </Callout>
        <div className={css.buttons}>
          <Button intent={Intent.PRIMARY} text="Click me" />
        </div>
      </div>
      <div className={css.main}>
        <MapWithNoSSR />
      </div>
    </div>
  )
}

export default HomePage
