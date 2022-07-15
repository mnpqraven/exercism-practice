// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */
  export class Size {
    constructor(width,height) {
      if (width) this.width = width
      else this.width = 80
      if (height) this.height = height
      else this.height = 60
    }

    // setter
    resize(width,height) {
      this.width = width
      this.height = height
    }
  }

export class Position {
  constructor (x,y) {
    if (x) this.x = x
    else this.x = 0
    if (y) this.y = y
    else this.y = 0
  }

  // setter
  move (x,y) {
    this.x = x
    this.y = y
  }
}

export class ProgramWindow {
  constructor() {
    this.screenSize = new Size(800,600)
    this.size =  new Size
    this.position = new Position
  }
  resize (x,y) {
    this.size.width = x
    this.size.height = y
  }
  move(x,y) {
    this.position.x = x
    this.position.y = y
  }
}

