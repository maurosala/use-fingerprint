import { useState, useEffect } from 'react'

import init, { fingerprint } from './pkg/fingerprint'

export default function useFingerprint() {
  const [f, setF] = useState({
    fingerprint: null,
    browser: null,
    ms: null
  })

  useEffect(() => {
    init().then(() => {
      const rust = fingerprint()
      console.log(rust)

      setF({ ...JSON.parse(rust) })
    })
  }, [])

  return f
}
