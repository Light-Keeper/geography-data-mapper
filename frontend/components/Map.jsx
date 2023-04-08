import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { useEffect, useMemo, useState } from "react";
import L from 'leaflet'
import { useDatapoints } from '../models/datapoints'
import { Colors } from '@blueprintjs/core'
import * as React from 'react'
import { dataUrlForIcon } from '../lib/icon'
import { IconNames } from '@blueprintjs/icons'
import css from './map.module.scss'

function defaultColoringFunction(d) {
  let colorClass = d.tags.Color;
  if (!colorClass && d.tags.Population) {
    let p = d.tags.Population;

    if (p > 10_000_000) {
      colorClass = 'red'
    } else if (p > 3_000_000) {
      colorClass = 'orage'
    } else if (p > 900_000) {
      colorClass = 'yellow'
    } else {
      colorClass = 'blue'
    }
  }

  let color =
    {
      red: Colors.RED4,
      blue: Colors.BLUE4,
      yellow: Colors.GOLD4,
      green: Colors.GREEN4,
      orange: Colors.ORANGE1,
    }[colorClass] || Colors.GOLD3

  return color;
}

function buildMarker(d, coloringFunction) {
  const { Name, ...rest } = d.tags

  coloringFunction = coloringFunction || defaultColoringFunction;
  let color = coloringFunction(d);

  let icon = L.icon({
    iconUrl: dataUrlForIcon({ icon: IconNames.MAP_MARKER, color: color }),
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

const PointsLayer = ({ selectedDatasource, limit, order, coloringFunction }) => {
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
    let markers = data.map(d => buildMarker(d, coloringFunction))
    markers.forEach((m) => m.addTo(map))
    return () => markers.forEach((m) => m.remove())
  }, [map, data, coloringFunction])

  useEffect(() => {
    map.on('moveend', () => setBbox(map.getBounds()));
  }, [map])
  return null
}

const centerOfUkraine = [49.026638, 31.482904]

const Map = ({ selectedDatasource, limit, order, coloringFunction }) => {
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
      {selectedDatasource && <PointsLayer selectedDatasource={selectedDatasource} limit={limit} order={order} coloringFunction={coloringFunction}/>}
    </MapContainer>
  )
}

export default Map
