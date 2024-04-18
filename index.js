import { useState, useEffect } from 'react'

import init, { fingerprint } from './pkg/fingerprint'

export default function useFingerprint() {
  const [f, setF] = useState({
    id: null,
    ms: null
  })

  const browser = () => {
    const browserFingerprint = [
      navigator.userAgent,
      navigator.language,
      navigator.maxTouchPoints,
      navigator.hardwareConcurrency,
      navigator.cookieEnabled,
      window.screen.width,
      window.screen.height,
      window.screen.colorDepth,
      window.screen.deviceXDPI,
      window.screen.deviceYDPI,
      window.screen.availWidth,
      window.screen.availHeight,
      window.screen.orientation,
      window.screen.pixelDepth
    ]
      .filter((x) => x)
      .join('|')

    return btoa(browserFingerprint)
  }

  useEffect(() => {
    init().then(() => {
      setF({ ...JSON.parse(fingerprint()), browser: browser() })
    })
  }, [])

  return f
}
