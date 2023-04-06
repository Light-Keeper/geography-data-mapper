import useSwr from 'swr'
import { Toaster } from '@blueprintjs/core'
import { apiRequest } from "../lib/api";

export async function getDatasources() {
  const page = await apiRequest('GET', '/api/datasets')
  return page.data;
}

const ssr = typeof window === 'undefined'
const toast = !ssr && Toaster.create({ className: 'z-index-1000' })

export function useDatasources() {
  return useSwr('/api/datasets', getDatasources, {
    onError: (err) => {
      console.log(err)
      toast.show({ message: 'Failed to load datasources', intent: 'danger' })
    },
  })
}
