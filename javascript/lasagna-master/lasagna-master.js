/// <reference path="./global.d.ts" />
// @ts-check

/**
  * @param {any} timer time left in the oven
  * @returns {str}
  */
export function cookingStatus(timer) {
  let str
  switch (timer) {
    case 0: str =  'Lasagna is done.';
      break;
    case undefined: str = 'You forgot to set the timer.'
      break;
    default: str = 'Not done, please wait.'
      break;
  }
  return str
}

/**
  * @param {any} layers arrays of items that needs preparing
  * @param {any} avgPrepTime 2 if not stated
  * @returns {number} product of # in layers and avgPrepTime
  */
export function preparationTime(layers, avgPrepTime) {
  let len = layers.length
  return (avgPrepTime === undefined) ? len * 2 :  len * avgPrepTime
}

       /**
        * @param {Array} layers list of items
        * @returns {any} object with keys 'noodles' and 'sauce'
        */
export function quantities(layers) {
  let noodleWeight=0
  let sauceVolume=0.0
  for (let i = 0; i < layers.length; i++) {
    if (layers[i] === 'noodles') noodleWeight += 50
    if (layers[i] === 'sauce') sauceVolume += 0.2
  }
  return {noodles: noodleWeight, sauce: sauceVolume}
}

/**
  * @param {Array} arrays1 [TODO:parameter]
  * @param {Array} arrays2 [TODO:parameter]
  */
export function addSecretIngredient(arrays1, arrays2) {
  arrays2.push(arrays1.at(arrays1.length-1))
}

/**
  * @param {Object} recipe amount of ingredients for **two**
  * @param {Number} portions number of portins
  * @returns {any} new scaled recipe
  */
export function scaleRecipe(recipe, portions) {
  let newRecipe = {}
  for (const key in recipe) {
    newRecipe[key] = recipe[key]*portions/2
  }
  return newRecipe
}
