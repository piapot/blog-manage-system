import { defineConfig } from "vite"
import { resolve } from "path"
import react from "@vitejs/plugin-react"
import Unocss from "unocss/vite"

export default defineConfig({
  plugins: [react(), Unocss()],
  resolve: { alias: { "@/": `${resolve(__dirname, "src")}/` } },
})
