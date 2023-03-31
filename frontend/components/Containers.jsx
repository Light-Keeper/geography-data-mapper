import styled from "styled-components";


export const AppLayout = styled(({ children, className }) => {
  return <div className={className}>{children}</div>
})`
  display: flex;
  height: 100vh;
  width: 100vw;
  background-color: #f5f5f0;
`

export const Sidebar = ({ children }) => {
  return <div>{children}</div>
}

export const MainContent = ({ children }) => {
  return <div>{children}</div>
}
