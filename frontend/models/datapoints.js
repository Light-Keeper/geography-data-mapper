import useSwr from 'swr'
import { getFakeDatapoints } from './fake'
import { Toaster } from '@blueprintjs/core'

export async function getDatapoints({ datasourceId }) {
  //return fetch(`/api/datapoints/${datasourceId}`).then((res) => res.json())
  return getFakeDatapoints(datasourceId)
}

let toastCache = null
const toast = () => toastCache ||= Toaster.create({ className: 'z-index-1000' })


export function useDatapoints({ datasourceId }) {
  return useSwr(`/api/datapoints/${datasourceId}`, () => getDatapoints({ datasourceId }), {
    onError: (err) => {
      console.error(err)
      toast().show({ message: `Failed to load datapoints for datasourceId=${datasourceId}`, intent: 'danger' })
    },
  })
}
