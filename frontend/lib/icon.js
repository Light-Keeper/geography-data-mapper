import * as React from 'react'
import { iconNameToPathsRecordKey, IconSvgPaths20 } from '@blueprintjs/icons'
import { renderToString } from 'react-dom/server'

let cache = new Map()

export function dataUrlForIcon({ icon, color }) {
  let cacheKey = `${icon}-${color}`
  if (!cache.has(cacheKey)) {
    cache.set(cacheKey, createDataUrlForIcon({ icon, color }))
  }
  return cache.get(cacheKey)
}

function createDataUrlForIcon({ icon, color }) {
  const pathsRaw = IconSvgPaths20[iconNameToPathsRecordKey(icon)]
  const paths = pathsRaw.map((path, i) => <path key={i} d={path} fillRule='evenodd' clipRule="evenodd" />)

  const svg = (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      x='0px'
      y='0px'
      fill={color}
      data-icon={icon}
      width={20}
      height={20}
      viewBox='0 0 20 20'
      role='img'
      enableBackground='new 0 0 20 20'
    >
      {paths}
    </svg>
  )

  const str = renderToString(svg)
  return `data:image/svg+xml;base64,${btoa(str)}`
}
