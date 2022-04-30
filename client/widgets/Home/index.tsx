import type { NextPage } from "next"

const Home: NextPage = () => {
  return (
    <main className='bg-white rounded-t-xl flex shadow-lg mx-8 p-4 pb-0 shadow-gray-300 w-1/2'>
      <ul className='flex bg-yellow-200 flex-1'>
        <li h='200vh' className='flex flex-col w-full items-center'>
          home
        </li>
      </ul>
    </main>
  )
}

export default Home
