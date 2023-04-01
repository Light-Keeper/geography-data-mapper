import styled from 'styled-components'
import { MapContainer, TileLayer } from 'react-leaflet'

const Map = styled(({ className }) => {
  return (
    <MapContainer center={[51.505, -0.09]} zoom={13} scrollWheelZoom className={className}>
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, &copy; <a href="https://cartodb.com/attributions">CartoDB</a>'
        url='https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}.png'
      />
    </MapContainer>
  )
})`
  width: 100%;
  height: 100%;
`

export default Map
