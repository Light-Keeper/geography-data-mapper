import useSwr from 'swr'
import { Toaster } from '@blueprintjs/core'
import { apiRequest } from "../lib/api";

export async function getDatapoints({ datasourceId }) {
  const page = await apiRequest('GET', '/api/datapoints?dataset=' + datasourceId)
  console.log(page)
  return page.data;
}

const ssr = typeof window === 'undefined'
const toast = !ssr && Toaster.create({ className: 'z-index-1000' })

export function useDatapoints({ datasourceId }) {
  return useSwr(`/api/datapoints/${datasourceId}`, () => getDatapoints({ datasourceId }), {
    onError: (err) => {
      console.error(err)
      toast.show({ message: `Failed to load datapoints for datasourceId=${datasourceId}`, intent: 'danger' })
    },
  })
}
