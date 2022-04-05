import type { FC } from "react"
import { Link } from "react-router-dom"
import { Paths } from "@/routes"

const NotFound: FC = () => {
  return (
    <main className="w-screen min-h-screen flex flex-col justify-center items-center">
      <p>404 - Not Found</p>
      <p>
        <Link
          to={Paths.INDEX}
          className="text-blue-500 hover:text-blue-400 active:text-blue-300 underline"
        >
          /index
        </Link>
      </p>
    </main>
  )
}

export default NotFound
