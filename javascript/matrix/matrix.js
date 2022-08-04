//
// This is only a SKELETON file for the 'Matrix' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class Matrix {
  constructor(str) {
      this.str =  str.split('\n')
          .map(row => {
              return row.split(' ').map(Number)
          })
  }

  get rows() {
      return this.str
  }

  get columns() {
      // NOTE: str[0] is the header for index
      // first map populates header to span array
      return this.str[0].map((col, index) => {
          // NOTE: 2nd map interates through rows and grab element
          return this.str.map(row => row[index])
      })
  }
}
