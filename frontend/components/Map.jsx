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

const PointsLayer = () => {
  const map = useMap()
  const { data } = useDatapoints({ datasourceId: 'test' })

  useEffect(() => {
    if (!data) return

    let markers = data.map((d) => {
      return L.marker([d.lat, d.lng], {
        icon: Math.random() < 0.5 ? blueIcon : yellowIcon,
        title: d.title
      })
    })

    markers.forEach((m) => m.addTo(map))
    return () => markers.forEach((m) => m.remove())
  }, [map, data])

  return null
}

const Map = () => {
  return (
    <MapContainer center={[49.026638, 31.482904]} zoom={6} scrollWheelZoom className={css.map}>
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, &copy; <a href="https://cartodb.com/attributions">CartoDB</a>'
        url='https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png'
      />
      <PointsLayer />
    </MapContainer>
  )
}

export default Map
