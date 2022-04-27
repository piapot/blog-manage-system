import type { AppProps } from "next/app"
import "windi.css"

function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />
}

export default App
