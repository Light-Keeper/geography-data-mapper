import styled from 'styled-components'
import { Colors } from '@blueprintjs/core'

export const AppLayout = styled(({ children, className }) => {
  return (
    <div className={className} data-name='AppLayout'>
      {children}
    </div>
  )
})`
  display: flex;
  height: 100vh;
  width: 100vw;
  background-color: ${Colors.LIGHT_GRAY5};
`

export const Sidebar = styled(({ children, className }) => {
  return <div className={className}>{children}</div>
})`
  display: flex;
  width: 20em;
  height: 100%;
  border: 1px solid ${Colors.GRAY5};
`

export const MainContent = styled(({ children, className }) => {
  return <div className={className}>{children}</div>
})`
  width: 100%;
  height: 100%;
`
