import styled from 'styled-components'
import { MapContainer, TileLayer, useMap } from "react-leaflet";
import { useEffect } from "react";
import L from 'leaflet'

const PointsLayer = () => {
  const map = useMap();
  useEffect(() => {
    let markerOptions = { icon: L.icon({
        iconUrl: '/icons/flag.svg',
        iconSize: [20, 20],
      }),

    };

    L.marker([49.026638, 31.482904], markerOptions).addTo(map);
  }, [map]);

  return null;
}

const Map = styled(({ className }) => {
  return (
    <MapContainer center={[49.026638, 31.482904]} zoom={6} scrollWheelZoom className={className}>
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, &copy; <a href="https://cartodb.com/attributions">CartoDB</a>'
        url='https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png'
      />
      <PointsLayer />
    </MapContainer>
  )
})`
  width: 100%;
  height: 100%;
`

export default Map
