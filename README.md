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
  "browser": "TW96aWxsYS81LjAgKGlQaG9uZTsgQ1BVIGlQaG9uZSBPUyAxNl82IGxpa2UgTWFjIE9TIFgpIEFwcGxlV2ViS2l0LzYwNS4xLjE1IChLSFRNTCwgbGlrZSBHZWNrbykgVmVyc2lvbi8xNi42IE1vYmlsZS8xNUUxNDggU2FmYXJpLzYwNC4xfGVuLUdCfDh8dHJ1ZXwzOTB8ODQ0fDI0fDM5MHw4NDR8W29iamVjdCBTY3JlZW5PcmllbnRhdGlvbl18MjQ=",
  "id": "B48D36C2",
  "ms": 20.4
}
```
