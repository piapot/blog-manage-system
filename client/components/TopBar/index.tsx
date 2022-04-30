import type { NextPage } from "next"

const TopBar: NextPage = () => {
  return (
    <header className='bg-white flex h-16 shadow-sm min-h-16 px-32 top-0 shadow-gray-300 z-9999 sticky justify-between items-center'>
      <h1 className='font-bold text-3xl'>Logo</h1>
      <ul>
        <li>user</li>
      </ul>
    </header>
  )
}

export default TopBar
