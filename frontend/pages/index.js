import { AppLayout, Sidebar, MainContent } from '../components/Containers'
import { MapWithNoSSR } from '../components/MapWithNoSSR'
import { Callout, Intent } from "@blueprintjs/core";

function HomePage() {
  return (
    <AppLayout>
      <Sidebar>
        <Callout title="Welcome to the Map" icon="info-sign" intent={Intent.PRIMARY}>
          <p>
            Experience the joy of discovering 100 random locations across Ukraine.
          </p>
        </Callout>
      </Sidebar>
      <MainContent>
        <MapWithNoSSR />
      </MainContent>
    </AppLayout>
  )
}

export default HomePage
