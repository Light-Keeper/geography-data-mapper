import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { useEffect } from 'react'
import L from 'leaflet'
import { useDatapoints } from '../models/datapoints'
import { Colors } from '@blueprintjs/core'
import * as React from 'react'
import { dataUrlForIcon } from '../lib/icon'
import { IconNames } from '@blueprintjs/icons'
import css from './map.module.scss'

const blueIcon = L.icon({
  iconUrl: dataUrlForIcon({ icon: IconNames.MAP_MARKER, color: Colors.BLUE4 }),
  iconSize: [20, 20],
})

const yellowIcon = L.icon({
  iconUrl: dataUrlForIcon({ icon: IconNames.MAP_MARKER, color: Colors.GOLD4 }),
  iconSize: [20, 20],
})

const grayIcon = L.icon({
  iconUrl: dataUrlForIcon({ icon: IconNames.MAP_MARKER, color: Colors.GRAY4 }),
  iconSize: [20, 20],
})

const PointsLayer = ({ selectedDatasource }) => {
  const map = useMap()
  const { data } = useDatapoints({ datasourceId: selectedDatasource.id })

  useEffect(() => {
    if (!data) return

    let markers = data.map((d) => {
      let icon = grayIcon
      if (d.color === 'blue') {
        icon = blueIcon
      }
      if (d.color === 'yellow') {
        icon = yellowIcon
      }

      let tooltip = '<b>' + d.name + '</b>'
      Object.entries(d.tags)
        .map(([k, v]) => `</br>${k}: ${v}`)
        .forEach((t) => (tooltip += t))

      return L.marker([d.lat, d.lng], {
        icon: icon,
        title: d.name,
      }).bindTooltip(tooltip)
    })

    markers.forEach((m) => m.addTo(map))
    return () => markers.forEach((m) => m.remove())
  }, [map, data])

  return null
}

const centerOfUkraine = [49.026638, 31.482904]

const Map = ({ selectedDatasource }) => {
  return (
    <MapContainer center={centerOfUkraine} zoom={6} scrollWheelZoom className={css.map}>
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, &copy; <a href="https://cartodb.com/attributions">CartoDB</a>'
        url='https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png'
      />
      {selectedDatasource && <PointsLayer selectedDatasource={selectedDatasource} />}
    </MapContainer>
  )
}

export default Map
