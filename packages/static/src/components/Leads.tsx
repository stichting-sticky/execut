import { useEffect, useRef, useState } from 'react'

import Scanner from 'qr-scanner'

import Results from '~/components/Results'
import Snackbar from '~/components/Snackbar'
import Verify from '~/components/Verify'

import { scanBadge, useCredentials, type Exhibitor } from '~/lib'

export default () => {
  const ref = useRef<HTMLVideoElement>(null)

  const [credentials, setCredentials] = useCredentials()

  const [showVerify, setShowVerify] = useState(false)
  const [showScans, setShowScans] = useState(false)
  const [messages, setMessages] = useState<string[]>([])
  const [badge, setBadge] = useState('')

  const handleScan = (badge: string) => {
    // if (badge.length !== 36) return

    if (!credentials && !showVerify) {
      setShowVerify(true)
      setBadge(badge)
    }

    if (credentials) {
      scanBadge(credentials, badge)
        .then((scan) => {
          const { subject } = scan

          if (credentials.role === 'attendee') {
            const { company } = subject as Exhibitor

            location.assign(`/partners/${company}`)
          } else if (credentials.role === 'exhibitor') {
            const { name } = subject

            setMessages((messages) => [`Scanned ${name}'s badge`, ...messages])
          }
        })
        .catch(() => {
          setMessages((messages) => ['Unable to scan badge', ...messages])
        })
    }
  }

  useEffect(() => {
    if (!ref.current) return

    const scanner = new Scanner(
      ref.current,
      ({ data }) => {
        handleScan(data)
      },
      { maxScansPerSecond: 1 },
    )

    scanner.start()

    return () => {
      scanner.stop()
    }
  }, [ref, showScans])

  return (
    <>
      {!showScans && (
        <>
          <video className="h-screen w-screen" ref={ref} />

          <div className="absolute inset-x-0 bottom-0">
            {messages.map((message, index) => (
              <Snackbar key={index}>{message}</Snackbar>
            ))}

            {!credentials && !badge && (
              <Snackbar delay="infinite">Scan your own badge first</Snackbar>
            )}

            {!credentials && badge && (
              <Verify
                badge={badge}
                setCredentials={setCredentials}
                setMessages={setMessages}
              />
            )}

            {credentials?.role === 'exhibitor' && (
              <button
                className="m-2 mt-0 h-12  bg-white/20 p-2 leading-8 text-white shadow-lg backdrop-blur"
                onClick={() => setShowScans(true)}>
                Show scans
              </button>
            )}
          </div>
        </>
      )}

      {showScans && (
        <>
          <button
            className="m-2 bg-red p-2 font-mono leading-8 text-white hover:bg-red/80 hover:drop-shadow focus:ring focus:ring-red/40"
            onClick={() => setShowScans(false)}>
            Back
          </button>

          <Results />
        </>
      )}
    </>
  )
}
