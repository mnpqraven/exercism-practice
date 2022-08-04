//
// This is only a SKELETON file for the 'Pangram' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const isPangram = (text) => {
    text = String(text).toLowerCase()
    return [...PANGRAM].every(char => [...text].includes(char))
};

export const PANGRAM = new Set("thequickbrownfoxjumpsoverthelazydog".split(''))
