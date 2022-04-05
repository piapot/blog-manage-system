import Index from "@/pages/Index"
import NotFound from "@/pages/NotFound"

export enum Paths {
  INDEX = "/",
}

export const routes = [
  {
    path: Paths.INDEX,
    el: <Index />,
  },
  {
    path: "*",
    el: <NotFound />,
  },
]
