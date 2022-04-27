import type { NextPage } from "next"

const Index: NextPage = () => {
  return (
    <main display='flex' flex='col' min-h='screen' w='screen' justify='center' items='center'>
      <button text='3xl blue-500 active:yellow-500' cursor='pointer'>
        Hello World!
      </button>
    </main>
  )
}

export default Index
