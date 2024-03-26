import { useEffect, useRef, useState } from 'react'

import { authorize, type Credentials } from '~/lib'

interface Props {
  badge: string
  setCredentials: React.Dispatch<React.SetStateAction<Credentials>>
  setMessages: React.Dispatch<React.SetStateAction<string[]>>
}

export default (props: Props) => {
  const { badge, setCredentials, setMessages } = props

  const ref = useRef<HTMLInputElement>(null)

  const [disabled, setDisabled] = useState(false)
  const [token, setToken] = useState('')

  const handler = async (badge: string, token: string) => {
    setToken(token.replace(/\D/g, ''))

    if (token.length !== 6) return

    setDisabled(true)

    authorize(badge, token)
      .then((credentials) => {
        setCredentials(credentials)
      })
      .catch(() => {
        setToken('')
        setDisabled(false)
        setMessages((messages) => ['Invalid combination of badge and token', ...messages])
      })
  }

  useEffect(() => {
    setToken(token.replace(/\D/g, ''))

    if (token.length !== 6) return

    setDisabled(true)
  }, [token])

  return (
    <div className="m-2 mt-0 bg-white/80 p-2 backdrop-blur shadow-lg">
      <label>
        <h1 className="font-mono text-3xl font-bold text-red sm:text-4xl">
          Token
        </h1>
        <input
          className="mt-4 block w-full appearance-none rounded-none border-0 border-b-2 border-subtle-highlight p-2 leading-6 outline-none focus:border-brown focus:ring-0 sm:mt-8 text-brown"
          disabled={disabled}
          ref={ref}
          onChange={({ target }) => { handler(badge, target.value) }}
          type="text"
          value={token}
        />
      </label>

      <p className="mt-4 sm:mt-8 text-brown">
        In the last mail you have received from us you can find a 6 digit token,
        we require you to fill this in to ensure that it's really you.
      </p>
    </div>
  )
}
