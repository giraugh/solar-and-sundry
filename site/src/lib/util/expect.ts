const expect = <T>(t: T | undefined | null, errorMessage: string): T => {
  if (t === undefined || t === null)
    throw new Error(errorMessage)
  return t
}

export default expect
