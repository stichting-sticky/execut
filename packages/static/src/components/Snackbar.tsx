import { useEffect, useState } from 'react'

interface Props {
  children: React.ReactNode
  delay?: number | 'infinite'
}

export default (props: Props) => {
  const { children, delay = 3000 } = props

  const [visible, setVisible] = useState(true)

  useEffect(() => {
    if (delay === 'infinite') return

    setTimeout(() => {
      setVisible(false)
    }, delay)
  })

  return ( visible && (
      <div className="m-2 mt-0 h-12 bg-green/60 p-2 leading-8 text-white shadow-lg backdrop-blur">
        {children}
      </div>
    )
  )
}
