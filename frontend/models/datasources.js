import useSwr from 'swr'
import { Toaster } from '@blueprintjs/core'
import { getFakeDatasources } from './fake'

export async function getDatasources() {
   //return fetch('/api/datasources').then(res => res.json())
  return getFakeDatasources()
}

const ssr = typeof window === 'undefined'
const toast = !ssr && Toaster.create({ className: 'z-index-1000' })

export function useDatasources() {
  return useSwr('/api/datasources', getDatasources, {
    onError: (err) => {
      console.log(err)
      toast.show({ message: 'Failed to load datasources', intent: 'danger' })
    },
  })
}
