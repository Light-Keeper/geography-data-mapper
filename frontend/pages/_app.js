import 'normalize.css/normalize.css'
import 'leaflet/dist/leaflet.css'

import '@blueprintjs/core/lib/css/blueprint.css'
import '@blueprintjs/icons/lib/css/blueprint-icons.css'

import { useEffect } from 'react'
import { FocusStyleManager } from '@blueprintjs/core'

function MyApp({ Component, pageProps }) {
  useEffect(() => {
    FocusStyleManager.onlyShowFocusOnTabs()
  }, [])

  return <Component {...pageProps} />
}

export default MyApp
