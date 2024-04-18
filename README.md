# React useFingerprint hook

A simple React hook to get canvas fingerprint and browser signature.

## Installation

```bash
npm install @maurosala/use-fingerprint
```

## Usage

```jsx
import useFingerprint from '@maurosala/use-fingerprint'

const fp = useFingerprint()
console.log(fp)
```

output:

```json
{
  "browser": "dd5b420791fe9a7dc9ebd8deb648692f76d68b839b3d04cae30702f76c31bc14",
  "id": "B48D36C2",
  "ms": 20.4
}
```

- `browser`: browser signature
- `id`: canvas fingerprint
- `ms`: time in milliseconds to get the fingerprint
