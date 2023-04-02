import 'normalize.css/normalize.css'
import 'leaflet/dist/leaflet.css'
import '@blueprintjs/core/lib/css/blueprint.css'
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css'
import '@blueprintjs/select/lib/css/blueprint-select.css'

import './golbal.css'

import { useEffect } from 'react'
import { FocusStyleManager } from '@blueprintjs/core'

function MyApp({ Component, pageProps }) {
  useEffect(() => {
    FocusStyleManager.onlyShowFocusOnTabs()
  }, [])

  return <Component {...pageProps} />
}

export default MyApp
