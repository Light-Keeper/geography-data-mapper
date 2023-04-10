import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { useEffect, useMemo, useState } from "react";
import L from 'leaflet'
import { useDatapoints } from '../models/datapoints'
import * as React from 'react'
import { dataUrlForIcon } from '../lib/icon'
import { IconNames } from '@blueprintjs/icons'
import css from './map.module.scss'
import { pickColoringStrategy } from "../lib/coloring";

function buildMarker(d) {
  const { Name, ...rest } = d.tags

  let icon = L.icon({
    iconUrl: dataUrlForIcon({ icon: IconNames.MAP_MARKER, color: d.color }),
    iconSize: [20, 20],
  })

  let tooltip = '<b>' + Name + '</b>'
  Object.entries(rest)
    .map(([k, v]) => `</br>${k}: ${v}`)
    .forEach((t) => (tooltip += t))

  return L.marker([d.lat, d.lng], {
    icon: icon,
    title: d.name,
  }).bindTooltip(tooltip)
}

const PointsLayer = ({ selectedDatasource, limit, order }) => {
  const map = useMap()
  const [bbox, setBbox] = useState(map.getBounds());

  const filters = useMemo(() => ({
    lat_min: bbox.getSouth(),
    lat_max: bbox.getNorth(),
    lng_min: bbox.getWest(),
    lng_max: bbox.getEast()
  }), [bbox])

  const { data } = useDatapoints({
    datasourceId: selectedDatasource.id,
    filters,
    order,
    limit
  })

  useEffect(() => {
    if (!data) return
    const coloring = pickColoringStrategy(selectedDatasource, data);

    let markers = data
      .map((d) => ({ ...d, color: coloring(d) }))
      .map(d => buildMarker(d))
    markers.forEach((m) => m.addTo(map))
    return () => markers.forEach((m) => m.remove())
  }, [map, data, selectedDatasource])

  useEffect(() => {
    map.on('moveend', () => setBbox(map.getBounds()));
  }, [map])
  return null
}

const centerOfUkraine = [49.026638, 31.482904]

const Map = ({ selectedDatasource, limit, order }) => {
  return (
    <MapContainer
      center={centerOfUkraine}
      zoom={6}
      worldCopyJump
      scrollWheelZoom
      className={css.map}>
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, &copy; <a href="https://cartodb.com/attributions">CartoDB</a>'
        url='https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png'
      />
      {selectedDatasource && <PointsLayer selectedDatasource={selectedDatasource} limit={limit} order={order}/>}
    </MapContainer>
  )
}

export default Map
