// @ts-check

/**
 * Calculates the sum of the two input arrays.
 *
 * @param {number[]} array1
 * @param {number[]} array2
 * @returns {number} sum of the two arrays
 */
export function twoSum(array1, array2) {
  // no separator
  const num1 = array1.join('')
  const num2 = array2.join('')
  return Number(num1) + Number(num2)
}

/**
 * Checks whether a number is a palindrome.
 *
 * @param {number} value
 * @returns {boolean}  whether the number is a palindrome or not
 */
export function luckyNumber(value) {
  let str = String(value)
  for (let index = 0; index < str.length; index++) {
    if ( str.charAt(index) != str.charAt(str.length-index-1) )
      return false
  }
  return true
}

/**
 * Determines the error message that should be shown to the user
 * for the given input value.
 *
 * @param {string|null|undefined} input
 * @returns {string} error message
 */
export function errorMessage(input) {
  if (!input) return 'Required field'
  else return Number(input) ? '' : 'Must be a number besides 0'
}
