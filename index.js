import { useState, useEffect, useRef } from 'react'
import axios from 'axios'

import init, { fingerprint } from './pkg/fingerprint'

export default function useFingerprint() {
  const [f, setF] = useState(null)
  const initRef = useRef(false)

  useEffect(() => {
    if (initRef.current) return

    initRef.current = true
    init().then(() => {
      const rust = JSON.parse(fingerprint())
      // console.log(rust)

      axios
        .get('https://api.iristag.io?q=' + rust.id)
        .then((res) => {
          setF({ ...res.data, ...rust })
        })
        .catch((err) => {
          console.log(err)
          alert(err.message)
        })

      // const { font } = JSON.parse(rust)
      // const aaa = []
      // const aaa = font.split('|')
      // console.log([...new Set(aaa)].length)
    })
  }, [])

  return f
}
