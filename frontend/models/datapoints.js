import useSwr from 'swr'
import turf from 'turf'
import geodata from '../public/ne_110m_admin_0_countries.json'

let fakeDatapoints = null
export const getDatapoints = async ({ datasourceId }) => {
  if (!fakeDatapoints) {
    fakeDatapoints = generateFakeDatapoints()
  }

  return fakeDatapoints
}

function generateFakeDatapoints() {
  const datapoints = []
  for (let i = 0; i < 100; i++) {
    datapoints.push({
      id: i,
      datasource: { id: 'local-fake' },
      ...randomCoordinatesIn(
        /* bounding box of Ukraine */
        [22.1, 44.4, 40.2, 52.4],
      ),
      title: 'Fake datapoint ' + i,
      tags: {
        tag1: 'value1',
        tag2: 'value2',
      },
    })
  }
  return datapoints
}

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
  const ukraine = geodata.features.find(feature => feature.properties.ADMIN === 'Ukraine');
  const ukraineFeature = turf.polygon(ukraine.geometry.coordinates);
  const pointToCheck = turf.point([lng, lat]);
  return turf.inside(pointToCheck, ukraineFeature);
}

export function useDatapoints({ datasourceId }) {
  return useSwr(`/api/datapoints/${datasourceId}`, () => getDatapoints({ datasourceId }))
}
