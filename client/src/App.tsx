import { FC } from "react"
import { Routes, Route } from "react-router-dom"
import { routes } from "./routes"

const App: FC = () => {
  return (
    <Routes>
      {routes.map(({ path, el }) => (
        <Route key={path} path={path} element={el} />
      ))}
    </Routes>
  )
}

export default App
