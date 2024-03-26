import {
  useCallback,
  useLayoutEffect,
  useRef,
  useState,
  type Dispatch,
  type SetStateAction,
} from 'react'

const url = import.meta.env.PUBLIC_API_ENDPOINT

export type Role = 'attendee' | 'exhibitor'

export interface Attendee {
  id: string
  role: 'attendee'
  name: string
  mail: string
  linkedin?: string
  study?: string
  degree?: string
  institution?: string
  graduationYear?: string
}

export interface Exhibitor {
  id: string
  role: 'exhibitor'
  name: string
  mail: string
  company: string
}

export type User = Attendee | Exhibitor

export interface Credentials {
  token: string
  role: Role
}

export interface Scan {
  id: string
  initiator: User
  subject: User
  isExpunged: boolean
  scannedAt: Date
}

export interface Scans {
  active: Scan[],
  passive: Scan[],
}

export const useCredentials = (
  key = '2024/credentials',
): [
    Credentials | undefined,
    Dispatch<SetStateAction<Credentials>>,
    () => void,
  ] => {
  const initializer = useRef((key: string) => {
    const value = localStorage.getItem(key)
    return value && JSON.parse(value)
  })

  const [state, setState] = useState<Credentials | undefined>(() =>
    initializer.current(key),
  )

  useLayoutEffect(() => setState(initializer.current(key)), [key])

  const set: Dispatch<SetStateAction<Credentials>> = useCallback(
    (f) => {
      const credentials = typeof f === 'function'
        ? (f as Function)(state) as Credentials
        : f as Credentials
      const value = JSON.stringify(credentials)

      localStorage.setItem(key, value)
      setState(credentials)
    },
    [key, setState],
  )

  const remove = useCallback(() => {
    localStorage.removeItem(key)
    setState(undefined)
  }, [key, setState])

  return [state, set, remove]
}

export const authorize = async (badge: string, token: string) => {
  const response = await fetch(`${url}/auth`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ badge, token }),
  })

  const body = await response.json()

  if (!response.ok) return Promise.reject(body)

  return body as Credentials
}

export const scanBadge = async (credentials: Credentials, badge: string) => {
  const { token } = credentials

  const response = await fetch(`${url}/scans/${badge}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
  })

  const body = await response.json()

  if (!response.ok) return Promise.reject(body)

  return body as Scan
}

export const getScans = async (credentials: Credentials) => {
  const { token, role } = credentials

  if (role !== 'exhibitor') return

  const response = await fetch(`${url}/scans`, {
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
  })

  if (!response.ok) return

  const body = await response.json()

  return body as Scans
}
