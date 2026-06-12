export default function run(ctx) {
  var files = ctx.files
  if (files === null) return true
  var keys = Object.keys(files)
  for (var i = 0; i < keys.length; i++) {
    var p = keys[i]
    if (isBlockedMedia(p)) {
      return { ok: false, error: 'video rejected: ' + p }
    }
    if (!isAllowedPath(p)) {
      return { ok: false, error: 'not allowlisted: ' + p }
    }
  }
  return true
}

function endsWith(s, suffix) {
  return s.length >= suffix.length && s.slice(s.length - suffix.length) === suffix
}

function startsWith(s, prefix) {
  return s.length >= prefix.length && s.slice(0, prefix.length) === prefix
}

function isBlockedMedia(p) {
  if (endsWith(p, '.mkv') || endsWith(p, '.MKV')) return true
  if (endsWith(p, '.mp4') || endsWith(p, '.MP4')) return true
  if (endsWith(p, '.mov') || endsWith(p, '.MOV')) return true
  if (endsWith(p, '.avi') || endsWith(p, '.AVI')) return true
  return false
}

function isAllowedPath(p) {
  if (p === 'index.json') return true
  if (p === 'assets.manifest.json') return true
  if (startsWith(p, '.relay/assets/') && endsWith(p, '.ipfs')) return true
  if (startsWith(p, 'hooks/')) return true
  if (endsWith(p, '.md') || endsWith(p, '.MD')) return true
  if (endsWith(p, '.reel')) return true
  return false
}
