import useSwr from "swr";
import { Toaster } from "@blueprintjs/core";
import { getFakeDatasources } from "./fake";

export async function getDatasources() {
  // return fetch('/api/datasources').then(res => res.json())
  return getFakeDatasources();
}

export function useDatasources() {
  return useSwr('/api/datasources', getDatasources, {
    onError: err => {
      console.log(err)
      Toaster.create().show({ message: 'Failed to load datasources', intent: "danger" })
    }
  })
}
