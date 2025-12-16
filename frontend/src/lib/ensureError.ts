export default function ensureError(e: unknown) {
	if (e instanceof Error) return e
	return new Error(`Value error ${e}`)
}
