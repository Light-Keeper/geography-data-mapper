import geodata from '../public/ne_110m_admin_0_countries.json'
import turf from 'turf'

const datasources = [
  {
    id: 1,
    name: '80 random yellow pointers',
    count: 80,
    init: () => ({ tags: { color: 'yellow' } }),
  },
  {
    id: 2,
    name: '70 random blue pointers',
    count: 70,
    init: () => ({ tags: { color: 'blue' } }),
  },
  {
    id: 3,
    name: '100 random yellow-blue pointers',
    count: 100,
    init: () => ({ tags: { color: Math.random() > 0.5 ? 'yellow' : 'blue' } }),
  },
]

export const getFakeDatasources = () => {
  return datasources.map((d) => ({
    id: d.id,
    name: d.name,
  }))
}

const getFakeDatapointsNoCache = (id) => {
  const datasource = datasources.find((d) => d.id === id)
  if (!datasource) {
    return []
  }
  const { count, init } = datasource
  const datapoints = []
  for (let i = 0; i < count; i++) {
    const datapoint = init()
    datapoints.push({
      ...datapoint,
      id: i,
      name: `datapoint ${i}`,
      tags: {
        population: Math.round(Math.random() * 1000000),
      },
      ...randomCoordinatesIn(
        /* bounding box of Ukraine */
        [22.1, 44.4, 40.2, 52.4],
      ),
    })
  }
  return datapoints
}

const cache = {}

export const getFakeDatapoints = (id) => (cache[id] ||= getFakeDatapointsNoCache(id))

function randomCoordinatesIn([minLng, minLat, maxLng, maxLat]) {
  let attempt = {
    lng: minLng + Math.random() * (maxLng - minLng),
    lat: minLat + Math.random() * (maxLat - minLat),
  }
  while (!isInUkraine(attempt)) {
    attempt = {
      lng: minLng + Math.random() * (maxLng - minLng),
      lat: minLat + Math.random() * (maxLat - minLat),
    }
  }

  return attempt
}

// check if point is in Ukraine using geojson
function isInUkraine({ lng, lat }) {
  const ukraine = geodata.features.find((feature) => feature.properties.ADMIN === 'Ukraine')
  const ukraineFeature = turf.polygon(ukraine.geometry.coordinates)
  const pointToCheck = turf.point([lng, lat])
  return turf.inside(pointToCheck, ukraineFeature)
}
