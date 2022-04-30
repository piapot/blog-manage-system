import type { NextPage } from "next"
import { TopBar } from "@/components"
import { SideBar, Home, Trend } from "@/widgets"

const Index: NextPage = () => {
  return (
    <div className='flex flex-col min-h-screen flex-1 w-full'>
      <TopBar />
      <div className='flex bg-gray-200 flex-1 px-9 pt-4 justify-center'>
        <SideBar />
        <Home />
        <Trend />
      </div>
    </div>
  )
}

export default Index
