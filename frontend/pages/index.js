import { AppLayout, Sidebar, MainContent } from '../components/Containers'
import { MapWithNoSSR } from '../components/MapWithNoSSR'

function HomePage() {
  return (
    <AppLayout>
      <Sidebar />
      <MainContent>
        <MapWithNoSSR />
      </MainContent>
    </AppLayout>
  )
}

export default HomePage
